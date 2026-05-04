# WebSocket Test Scripts

Deze folder bevat test scripts voor de WebSocket implementatie.

## 🚀 Quick Start

### 1. Browser Test (Simpelst)
Open in je browser:
```
http://localhost:8080/websocket-test.html
```

### 2. Node.js Test
```bash
npm install ws
node test-websocket.js
```

Vergeet niet je API key aan te passen in het script!

### 3. Python Test
```bash
pip install websockets
python test-websocket.py
```

Vergeet niet je API key aan te passen in het script!

### 4. Command Line (wscat)
```bash
npm install -g wscat
wscat -c ws://localhost:8080/websocket

# Na connectie:
> {"action":"subscribe","api_key":"jouw-api-key"}
```

## 📝 Test Scenario

1. **Start de server:**
   ```bash
   cargo run
   ```

2. **Connecteer WebSocket** (kies één van de methoden hierboven)

3. **Maak een payment aan:**
   ```bash
   curl -X POST http://localhost:8080/payment/request \
     -H "X-API-Key: jouw-api-key" \
     -H "Content-Type: application/json" \
     -d '{"receive_address":"nano_test","amount":"1000000000000000000000000000000"}'
   ```

4. **Stuur een test betaling** naar het gegenereerde address

5. **Zie de WebSocket notificatie** verschijnen in je test client!

## ✅ Verwachte Output

**Bij connectie:**
```json
{"success":true,"message":"Subscribed successfully"}
```

**Bij betaling:**
```json
{
  "event": "transaction_confirmed",
  "data": {
    "transaction_id": "550e8400-e29b-41d4-a716-446655440000",
    "receive_address": "nano_...",
    "amount": "1000000000000000000000000000000",
    "hash": "ABC123...",
    "status": "confirmed"
  }
}
```

## 🐛 Troubleshooting

Zie [TEST_WEBSOCKET.md](TEST_WEBSOCKET.md) voor uitgebreide troubleshooting guide.
