# 🔒 Lokale WSS (WebSocket Secure) Setup

Voor test- en development omgeving met WSS (versleuteld), gebruik Caddy als reverse proxy.

## 🚀 Optie 1: Caddy (Aanbevolen - Makkelijkst!)

Caddy handelt automatisch SSL/TLS af, zelfs voor localhost!

### Installatie

**Windows:**
```powershell
# Via Chocolatey
choco install caddy

# Of download van: https://caddyserver.com/download
```

**Linux/Mac:**
```bash
# Linux
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt update
sudo apt install caddy

# Mac
brew install caddy
```

### Setup

1. **Start je Rust API** (poort 3000):
   ```bash
   cargo run
   ```

2. **Start Caddy** (terminal 2):
   ```bash
   caddy run
   ```

   Caddy leest automatisch de `Caddyfile` en:
   - ✅ Start op poort 8443 met TLS
   - ✅ Genereert automatisch self-signed certificaat
   - ✅ Proxy's alles naar localhost:3000

3. **Test de verbinding:**
   
   Eerst maak je een transactie aan:
   ```bash
   curl -X POST http://localhost:8080/api/transaction \
     -H "X-API-Key: jouw-key" \
     -H "Content-Type: application/json" \
     -d '{"receive_address":"nano_...","amount":"1"}'
   ```
   
   Dan test je WebSocket met de `transaction_id`:
   ```bash
   # Node.js/wscat
   wscat -c wss://localhost:8443/api/websocket/TRANSACTION_ID --no-check
   
   # Browser Console
   const ws = new WebSocket('wss://localhost:8443/api/websocket/TRANSACTION_ID');
   ws.onmessage = (e) => console.log(JSON.parse(e.data));
   ```

   **Let op:** Browser zal waarschuwen over self-signed certificate:
   1. Klik "Advanced" / "Geavanceerd"
   2. Klik "Proceed to localhost" / "Doorgaan naar localhost"

## 🚀 Optie 2: nginx met Self-Signed Certificate

### Genereer Certificate

```bash
# Maak certificates directory
mkdir -p certs
cd certs

# Genereer self-signed certificate
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout localhost.key \
  -out localhost.crt \
  -subj "/CN=localhost"

cd ..
```

### nginx Configuratie

Maak `nginx.conf`:

```nginx
events {
    worker_connections 1024;
}

http {
    upstream backend {
        server localhost:3000;
    }

    server {
        listen 8443 ssl;
        server_name localhost;

        ssl_certificate certs/localhost.crt;
        ssl_certificate_key certs/localhost.key;

        location /api/websocket {
            proxy_pass http://backend;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
        }

        location / {
            proxy_pass http://backend;
        }
    }
}
```

### Start nginx

```bash
nginx -c $(pwd)/nginx.conf
```

## 🧪 Testen

### Stap 1: Maak een transactie aan
```bash
# Maak eerst een transactie
curl -X POST http://localhost:3000/api/transaction \
  -H "X-API-Key: jouw-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "receive_address": "nano_your_address...",
    "amount": "1000000000000000000000000000000"
  }'

# Response bevat transaction_id:
# {"transaction_id":"550e8400-e29b-41d4-a716-446655440000",...}
```

### Stap 2: Test WebSocket Connectie

### Node.js
```javascript
const WebSocket = require('ws');

// Gebruik transaction_id uit stap 1
const transactionId = '550e8400-e29b-41d4-a716-446655440000';
const ws = new WebSocket(`wss://localhost:8443/api/websocket/${transactionId}`, {
    rejectUnauthorized: false // Voor self-signed cert
});

ws.on('open', () => console.log('✅ Connected'));
ws.on('message', (data) => console.log('📨', data.toString()));
```

### Python
```python
import asyncio
import websockets
import ssl

async def test():
    transaction_id = '550e8400-e29b-41d4-a716-446655440000'
    ssl_context = ssl.create_default_context()
    ssl_context.check_hostname = False
    ssl_context.verify_mode = ssl.CERT_NONE  # Voor self-signed cert
    
    uri = f'wss://localhost:8443/api/websocket/{transaction_id}'
    async with websockets.connect(uri, ssl=ssl_context) as ws:
        print('✅ Connected')
        async for message in ws:
            print(f'📨 {message}')

asyncio.run(test())
```

### Browser (JavaScript Console)
```javascript
// Gebruik transaction_id uit API response
const transactionId = '550e8400-e29b-41d4-a716-446655440000';
const ws = new WebSocket(`wss://localhost:8443/api/websocket/${transactionId}`);

ws.onopen = () => console.log('✅ Connected');
ws.onmessage = (e) => console.log('📨', JSON.parse(e.data));
```

### wscat
```bash
# Test WSS met transaction_id
wscat -c wss://localhost:8443/api/websocket/550e8400-e29b-41d4-a716-446655440000 --no-check

# Direct na connectie:
< {"success":true,"message":"Connected successfully"}

# Bij betaling:
< {"event":"transaction_confirmed","data":{"transaction_id":"550e8400...","status":"confirmed"}}
```

## 📊 Architectuur

```
Client (Browser/Node/Python)
    ↓ WSS (port 8443, TLS encrypted)
Caddy/nginx (Reverse Proxy)
    ↓ HTTP (port 3000, local only)
Rust API Server
    ↓
WebSocket per transaction_id
    ↓ Notificatie bij betaling
Client ontvangt "transaction_confirmed"
```

**Flow:**
1. Client maakt transactie aan via API → krijgt `transaction_id`
2. Client connecteert WebSocket: `wss://.../api/websocket/{transaction_id}`
3. Bij betaling → Nano node triggert → Server broadcast naar WebSocket
4. Client ontvangt real-time notificatie

## ✅ Verificatie

Check of WSS werkt:

```bash
# Test HTTPS endpoint
curl -k https://localhost:8443/api/stats

# Test WSS met wscat (gebruik een echte transaction_id)
wscat -c wss://localhost:8443/api/websocket/YOUR_TRANSACTION_ID --no-check
```

Je zou moeten zien:
- ✅ Caddy/nginx draait op poort 8443
- ✅ Rust API draait op poort 3000
- ✅ TLS handshake succesvol
- ✅ WebSocket upgrade werkt
- ✅ Direct na connectie: `{"success":true,"message":"Connected successfully"}`

## 🐛 Troubleshooting

### "Connection refused"
- Check of Caddy/nginx draait: `caddy version` of `nginx -v`
- Check of poort 8443 beschikbaar is: `netstat -an | findstr 8443`

### "Certificate error"
- Test scripts accepteren self-signed certs automatisch
- Browser: klik "Advanced" → "Proceed"
- wscat: gebruik `--no-check` flag

### "WebSocket upgrade failed"
- Verifieer dat Caddy/nginx correct proxy't
- Check Caddy logs: `caddy run` (toont logs in terminal)
- Check nginx logs: `nginx -V` voor log locaties

## 🎯 Productie

Voor productie gebruik je een echte domain naam:

**Caddyfile:**
```caddy
api.jouwdomein.com {
    reverse_proxy localhost:8080
}
```

Caddy haalt dan **automatisch** een gratis Let's Encrypt SSL certificaat op! 🎉

## 📚 Resources

- Caddy: https://caddyserver.com/docs/
- Let's Encrypt: https://letsencrypt.org/
- nginx SSL: https://nginx.org/en/docs/http/configuring_https_servers.html
