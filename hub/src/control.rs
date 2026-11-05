//! ConvCraft Native MCP Server & JSON Control-Channel Bridge
//!
//! Provides programmatic control of ConvCraft via:
//! 1. In-process AI Agent Sidebar tool execution (`ConvCraftApp::execute_command`)
//! 2. TCP JSON-lines Control Channel (`convcraft --control <port>`)
//! 3. Stdio Model Context Protocol (MCP) Server (`convcraft mcp` / `convcraft --mcp`)

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use serde_json::{Value, json};


pub struct ControlRequest {
    pub method: String,
    pub params: Value,
    pub reply: Sender<Value>,
}

/// Starts a background TCP listener for `--control <port>` speaking JSON lines.
pub fn start_control_server(port: u16) -> Result<Receiver<ControlRequest>, String> {
    let listener = TcpListener::bind(("127.0.0.1", port))
        .map_err(|e| format!("Failed to bind ConvCraft control server on 127.0.0.1:{port}: {e}"))?;
    let (tx, rx) = mpsc::channel::<ControlRequest>();

    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let tx_clone = tx.clone();
            std::thread::spawn(move || {
                handle_control_client(stream, tx_clone);
            });
        }
    });

    Ok(rx)
}

fn handle_control_client(stream: TcpStream, tx: Sender<ControlRequest>) {
    let Ok(mut writer) = stream.try_clone() else { return };
    let reader = BufReader::new(stream);
    for line in reader.lines().map_while(Result::ok) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parsed: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(e) => {
                let _ = writeln!(writer, "{}", json!({"ok": false, "error": format!("Invalid JSON: {e}")}));
                continue;
            }
        };
        let method = parsed
            .get("method")
            .or_else(|| parsed.get("command"))
            .and_then(Value::as_str)
            .unwrap_or("conv.inspectQueue")
            .to_string();
        let params = parsed.get("params").cloned().unwrap_or_else(|| parsed.clone());

        let (reply_tx, reply_rx) = mpsc::channel();
        if tx.send(ControlRequest { method, params, reply: reply_tx }).is_err() {
            break;
        }
        match reply_rx.recv_timeout(std::time::Duration::from_secs(10)) {
            Ok(resp) => {
                let _ = writeln!(writer, "{resp}");
                let _ = writer.flush();
            }
            Err(_) => {
                let _ = writeln!(writer, "{}", json!({"ok": false, "error": "Control request timed out"}));
                let _ = writer.flush();
            }
        }
    }
}

/// Runs stdio MCP server loop.
pub fn run_stdio_mcp_server(tx: Sender<ControlRequest>) {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lines().map_while(Result::ok) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Ok(req) = serde_json::from_str::<Value>(trimmed) else {
            let _ = writeln!(out, "{}", json!({"jsonrpc": "2.0", "error": {"code": -32700, "message": "Parse error"}}));
            let _ = out.flush();
            continue;
        };

        let id = req.get("id").cloned().unwrap_or(Value::Null);
        let method = req.get("method").and_then(Value::as_str).unwrap_or("");

        match method {
            "initialize" => {
                let resp = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "tools": {}
                        },
                        "serverInfo": {
                            "name": "convcraft-mcp",
                            "version": "1.0.0"
                        }
                    }
                });
                let _ = writeln!(out, "{resp}");
                let _ = out.flush();
            }
            "tools/list" => {
                let resp = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": mcp_tool_definitions()
                    }
                });
                let _ = writeln!(out, "{resp}");
                let _ = out.flush();
            }
            "tools/call" => {
                let tool_name = req.pointer("/params/name").and_then(Value::as_str).unwrap_or("");
                let args = req.pointer("/params/arguments").cloned().unwrap_or(json!({}));
                let (reply_tx, reply_rx) = mpsc::channel();
                if tx.send(ControlRequest { method: tool_name.to_string(), params: args, reply: reply_tx }).is_err() {
                    let _ = writeln!(out, "{}", json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {"code": -32603, "message": "ConvCraft engine disconnected"}
                    }));
                    let _ = out.flush();
                    continue;
                }
                match reply_rx.recv_timeout(std::time::Duration::from_secs(10)) {
                    Ok(val) => {
                        let is_err = val.get("ok").and_then(Value::as_bool) == Some(false);
                        let resp = json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "content": [{
                                    "type": "text",
                                    "text": serde_json::to_string_pretty(&val).unwrap_or_default()
                                }],
                                "isError": is_err
                            }
                        });
                        let _ = writeln!(out, "{resp}");
                        let _ = out.flush();
                    }
                    Err(_) => {
                        let _ = writeln!(out, "{}", json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": {"code": -32603, "message": "Tool execution timed out"}
                        }));
                        let _ = out.flush();
                    }
                }
            }
            _ => {
                let resp = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {"code": -32601, "message": format!("Method not found: {method}")}
                });
                let _ = writeln!(out, "{resp}");
                let _ = out.flush();
            }
        }
    }
}

pub fn mcp_tool_definitions() -> Value {
    json!([
        {
            "name": "conv.addFiles",
            "description": "Adds one or more media files or directories to the ConvCraft conversion queue.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "paths": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "List of absolute file or directory paths to add"
                    }
                },
                "required": ["paths"]
            }
        },
        {
            "name": "conv.clearQueue",
            "description": "Clears all items from the conversion queue (supports Undo).",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "required": []
            }
        },
        {
            "name": "conv.removeFile",
            "description": "Removes a file from the conversion queue by index or ID.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "index": {"type": "integer", "description": "0-based index of item to remove"},
                    "id": {"type": "string", "description": "Unique ID of item to remove"}
                },
                "required": []
            }
        },
        {
            "name": "conv.setTargetFormat",
            "description": "Sets the target output format for all items in the queue, or filtered by category.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "format": {
                        "type": "string",
                        "description": "Target format extension: webp, png, jpg, gif, bmp, mp3, wav, flac, aac, ogg, mp4, mkv, webm, mov, pdf"
                    },
                    "filter": {
                        "type": "string",
                        "description": "Optional category filter: image, audio, video, document, or all"
                    }
                },
                "required": ["format"]
            }
        },
        {
            "name": "conv.setCompressionAndQuality",
            "description": "Configures conversion quality (1-100), audio bitrate (e.g. '192k', '320k'), and video CRF (0-51).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "quality": {"type": "integer", "description": "Image/Video quality from 1 (lowest) to 100 (lossless)"},
                    "bitrate": {"type": "string", "description": "Audio bitrate string, e.g. '128k', '192k', '256k', '320k'"},
                    "crf": {"type": "integer", "description": "Video Constant Rate Factor (18=high, 23=default, 28=economy)"}
                },
                "required": []
            }
        },
        {
            "name": "conv.setResizeAndTransform",
            "description": "Configures image and video dimensions, scale percentage, rotation angle, and flipping.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "width": {"type": "integer", "description": "Target width in pixels"},
                    "height": {"type": "integer", "description": "Target height in pixels"},
                    "scale": {"type": "number", "description": "Scale factor (e.g. 0.5 for 50%, 2.0 for 200%)"},
                    "rotation": {"type": "integer", "description": "Rotation in degrees: 0, 90, 180, or 270"},
                    "flip": {"type": "string", "description": "Flip direction: 'none', 'horizontal', 'vertical', 'both'"}
                },
                "required": []
            }
        },
        {
            "name": "conv.applyVisualEffect",
            "description": "Applies a visual filter to images and videos (e.g. grayscale, blur, invert, sepia, brighten).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "effect": {
                        "type": "string",
                        "description": "Filter name: none, grayscale, blur, invert, sepia, brighten, contrast"
                    }
                },
                "required": ["effect"]
            }
        },
        {
            "name": "conv.startConversion",
            "description": "Starts conversion process for all ready items in the queue asynchronously.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "required": []
            }
        },
        {
            "name": "conv.exportToDrive",
            "description": "Copies all converted output files into Google Drive (G:\\My Drive\\Art Craft\\ConvCraft).",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "required": []
            }
        },
        {
            "name": "conv.saveAll",
            "description": "Saves current queue configuration and settings to disk.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "required": []
            }
        },
        {
            "name": "conv.inspectQueue",
            "description": "Returns current queue state, items, source/target formats, and progress.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "required": []
            }
        }
    ])
}
