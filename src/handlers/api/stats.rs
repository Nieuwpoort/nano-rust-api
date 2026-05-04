use axum::response::Html;
use crate::services::stats::stats_service::get_memory_stats;

pub async fn get_stats_api() -> Html<String> {
    let stats = get_memory_stats().await.0;
    
    let html = format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>x402 Nano Server Stats</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }}
        
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        
        h1 {{
            color: white;
            text-align: center;
            margin-bottom: 30px;
            font-size: 2.5em;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }}
        
        .stats-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            margin-bottom: 20px;
        }}
        
        .stat-card {{
            background: white;
            border-radius: 10px;
            padding: 25px;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
            transition: transform 0.3s ease;
        }}
        
        .stat-card:hover {{
            transform: translateY(-5px);
            box-shadow: 0 6px 12px rgba(0,0,0,0.15);
        }}
        
        .stat-header {{
            display: flex;
            align-items: center;
            margin-bottom: 15px;
        }}
        
        .stat-icon {{
            font-size: 2em;
            margin-right: 15px;
        }}
        
        .stat-title {{
            font-size: 0.9em;
            color: #666;
            text-transform: uppercase;
            letter-spacing: 1px;
        }}
        
        .stat-value {{
            font-size: 2.5em;
            font-weight: bold;
            color: #333;
        }}
        
        .stat-unit {{
            font-size: 0.5em;
            color: #999;
            margin-left: 5px;
        }}
        
        .stat-detail {{
            font-size: 0.9em;
            color: #777;
            margin-top: 10px;
        }}
        
        .refresh-btn {{
            display: block;
            margin: 20px auto;
            padding: 15px 40px;
            background: white;
            border: none;
            border-radius: 50px;
            font-size: 1em;
            font-weight: bold;
            color: #667eea;
            cursor: pointer;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
            transition: all 0.3s ease;
        }}
        
        .refresh-btn:hover {{
            transform: scale(1.05);
            box-shadow: 0 6px 12px rgba(0,0,0,0.15);
        }}
        
        .footer {{
            text-align: center;
            color: white;
            margin-top: 30px;
            opacity: 0.8;
        }}
        
        .status-good {{ color: #10b981; }}
        .status-warning {{ color: #f59e0b; }}
        .status-critical {{ color: #ef4444; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 x402 Nano Server Statistics</h1>
        
        <div class="stats-grid">
            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">💾</div>
                    <div class="stat-title">Memory Usage</div>
                </div>
                <div class="stat-value">{:.2}<span class="stat-unit">MB</span></div>
                <div class="stat-detail">{} bytes</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">⚡</div>
                    <div class="stat-title">CPU Usage</div>
                </div>
                <div class="stat-value">{:.2}<span class="stat-unit">%</span></div>
                <div class="stat-detail">{} cores available</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">⏱️</div>
                    <div class="stat-title">Uptime</div>
                </div>
                <div class="stat-value">{}</div>
                <div class="stat-detail">{} seconds</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">📊</div>
                    <div class="stat-title">Transactions/Second</div>
                </div>
                <div class="stat-value">{}<span class="stat-unit">TPS</span></div>
                <div class="stat-detail">Peak: {} TPS</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">📈</div>
                    <div class="stat-title">Transactions/Hour</div>
                </div>
                <div class="stat-value">{}<span class="stat-unit">TPH</span></div>
                <div class="stat-detail">Current hourly rate</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">🎯</div>
                    <div class="stat-title">Total Transactions</div>
                </div>
                <div class="stat-value">{}</div>
                <div class="stat-detail">All-time total processed</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">🗄️</div>
                    <div class="stat-title">Database Pool</div>
                </div>
                <div class="stat-value">{}<span class="stat-unit">/ {}</span></div>
                <div class="stat-detail">Active connections (Idle: {})</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">🔑</div>
                    <div class="stat-title">API Keys Cached</div>
                </div>
                <div class="stat-value">{}</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">💳</div>
                    <div class="stat-title">Pending Transactions</div>
                </div>
                <div class="stat-value">{}</div>
                <div class="stat-detail">In transaction cache</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-header">
                    <div class="stat-icon">💰</div>
                    <div class="stat-title">Nano Price</div>
                </div>
                <div class="stat-value">{}<span class="stat-unit">USD</span></div>
                <div class="stat-detail">€{} EUR • ₿{} BTC</div>
            </div>
        </div>
        
        <button class="refresh-btn" onclick="location.reload()">🔄 Refresh Stats</button>
        
        <div class="footer">
            <p>x402 Nano API Server • Real-time Monitoring Dashboard</p>
        </div>
    </div>
    
    <script>
        // Auto-refresh every 5 seconds
        setTimeout(() => location.reload(), 5000);
    </script>
</body>
</html>
    "#,
        stats.memory_used_mb,
        stats.memory_used_bytes,
        stats.cpu_usage_percent,
        stats.cpu_cores,
        stats.uptime_human,
        stats.uptime_seconds,
        stats.current_tps,
        stats.peak_tps,
        stats.current_tph,
        stats.total_transactions,
        stats.db_active_connections,
        stats.db_pool_size,
        stats.db_idle_connections,
        stats.cache_api_keys,
        stats.cache_transactions,
        stats.nano_price_usd.map(|p| format!("${:.4}", p)).unwrap_or_else(|| "N/A".to_string()),
        stats.nano_price_eur.map(|p| format!("{:.4}", p)).unwrap_or_else(|| "N/A".to_string()),
        stats.nano_price_btc.map(|p| format!("{:.8}", p)).unwrap_or_else(|| "N/A".to_string())
    );
    
    Html(html)
}
