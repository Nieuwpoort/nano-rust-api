# WebSocket Testen - Quick Guide

## 🎯 Hoe het werkt

Je huidige implementatie werkt **per transactie**:
1. Maak een transactie aan via API
2. Connecteer WebSocket met de `transaction_id`
3. Ontvang automatisch een notificatie wanneer de betaling binnenkomt

**Geen aparte subscribe actie nodig!**

## 🔥 Complete Test Scenario

### Stap 1: Start de server
```bash
cargo run
```

### Stap 2: Maak een transactie aan

```bash
curl -X POST http://localhost:3000/api/transaction \
  -H "X-API-Key: jouw-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "receive_address": "nano_1yourtopupaddress...",
    "amount": "1000000000000000000000000000000"
  }'
```

**Response:**
```json
{
  "transaction_id": "550e8400-e29b-41d4-a716-446655440000",
  "receive_address": "nano_...",
  "amount": "1000000000000000000000000000000"
}
```

### Stap 3: Connecteer WebSocket

**JavaScript (Browser Console):**
```javascript
const transactionId = '550e8400-e29b-41d4-a716-446655440000';
const ws = new WebSocket(`ws://localhost:3000/api/websocket/${transactionId}`);

ws.onopen = () => {
    console.log('✅ Connected to transaction WebSocket');
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('📨 Received:', data);
    
    if (data.event === 'transaction_confirmed') {
        console.log('💰 Payment confirmed!');
    }
};

ws.onerror = (error) => console.error('❌ Error:', error);
ws.onclose = () => console.log('🔌 Disconnected');
```

**wscat (Command Line):**
```bash
# Install wscat
npm install -g wscat

# Connecteer (vervang transaction_id)
wscat -c ws://localhost:3000/api/websocket/550e8400-e29b-41d4-a716-446655440000

# Direct na connectie krijg je:
< {"success":true,"message":"Connected successfully"}
```

**Python:**
```python
import asyncio
import websockets
import json

async def test_websocket():
    transaction_id = "550e8400-e29b-41d4-a716-446655440000"
    uri = f"ws://localhost:3000/api/websocket/{transaction_id}"
    
    async with websockets.connect(uri) as ws:
        print("✅ Connected")
        
        # Ontvang berichten
        async for message in ws:
            data = json.loads(message)
            print(f"📨 Received: {data}")
            
            if data.get('event') == 'transaction_confirmed':
                print("💰 Payment confirmed!")
                break

asyncio.run(test_websocket())
```

### Stap 4: Wacht op betaling

Wanneer de betaling binnenkomt via de Nano node, krijg je:

```json
{
  "event": "transaction_confirmed",
  "data": {
    "transaction_id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "confirmed"
  }
}
```

## 🧪 Testen met AI Agent (Lokaal)

### Setup
1. Start je Rust API: `cargo run`
2. Maak transactie aan via API
3. Geef de `transaction_id` aan je AI agent
4. AI agent connecteert naar WebSocket endpoint
5. Simuleer betaling via Nano node (of wacht op echte betaling)

### AI Agent Example Code

```javascript
// Voor je AI agent om te gebruiken
async function subscribeToTransaction(transactionId) {
    const ws = new WebSocket(`ws://localhost:3000/api/websocket/${transactionId}`);
    
    return new Promise((resolve, reject) => {
        ws.onmessage = (event) => {
            const data = JSON.parse(event.data);
            if (data.event === 'transaction_confirmed') {
                resolve(data);
                ws.close();
            }
        };
        
        ws.onerror = reject;
    });
}

// Gebruik:
const transactionId = 'krijg-je-van-api';
const result = await subscribeToTransaction(transactionId);
console.log('Payment confirmed!', result);
```

## 🐛 Troubleshooting

### Error: "Connection refused"
- Check of de server draait: `cargo run`
- Verifieer de poort in `.env`: `HTTP_LISTEN_ADDR`

### Geen berichten ontvangen
- Check of de `transaction_id` bestaat in de cache
- Verifieer dat de Nano node WebSocket werkend is (`WEB_SOCKET_BASE_URL` in `.env`)
- Check of het juiste payment address wordt gebruikt (`TOPUP_CREDITS_ADDRESS`)

### WebSocket disconnect direct na connectie
- Verifieer dat de `transaction_id` geldig en actief is
- Check of de transactie niet al expired is (standaard 6 minuten)

## 📊 Monitor Connections

Server logs tonen automatisch:
```
WebSocket manager initialized
WebSocket connection established for transaction: 550e8400...
Transaction confirmed: 550e8400...
WebSocket connection closed for transaction: 550e8400...
```

## ✅ Success Checklist

- [ ] Server start zonder errors
- [ ] Transactie aanmaken via API werkt
- [ ] WebSocket connectie succesvol met `transaction_id`
- [ ] Ontvang "Connected successfully" bericht
- [ ] Ontvang "transaction_confirmed" bij betaling
- [ ] Cleanup werkt (transactie verwijderd na 6 min of bij disconnect)
