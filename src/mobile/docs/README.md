# ifenpay Mobile Documentation

Documentation and integration guides for mobile applications.

## Overview

ifenpay provides APIs that work seamlessly with mobile applications. Whether you're building a native iOS/Android app or using React Native/Flutter, integrating Nano payments is straightforward.

## Mobile Integration Options

### 1. REST API
Use the same REST API as web applications:
- `POST /transaction/create` - Create payment
- `POST /transaction/pay` - Process payment
- `POST /payment/status/{id}` - Check status

### 2. Hosted Payment Pages
Redirect users to our mobile-optimized payment pages:
- Responsive design
- Touch-optimized buttons
- QR code scanning support

### 3. Deep Linking
Coming soon: Deep link integration for Nano wallet apps.

## Getting Started

1. Create your merchant API key at [ifenpay.com/merchant/key/create](https://ifenpay.com/merchant/key/create)
2. Follow the integration guide for your platform
3. Test payments in development mode

## Platform Guides

- [iOS Integration](./ios-integration.md) (Coming soon)
- [Android Integration](./android-integration.md) (Coming soon)
- [React Native](./react-native-integration.md) (Coming soon)
- [Flutter](./flutter-integration.md) (Coming soon)

## Support

Questions? Open an issue on [GitHub](https://github.com/ifenpay-com) or join our [Discord](https://discord.gg/CcxMmMuXM6).
