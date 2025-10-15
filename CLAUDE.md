# Continuebee - Planet Nine Authentication Service

## Overview

Continuebee is a Planet Nine allyabase microservice that provides centralized authentication and user verification with sessionless cryptography.

**Location**: `/continuebee/`
**Port**: 3001 (default)

## Core Features

### 🔐 **Authentication**
- **Sessionless Verification**: Cryptographic signature verification
- **Public Key Management**: Store and retrieve user public keys
- **User Registration**: Create authenticated users
- **Signature Validation**: Verify request signatures

## API Endpoints

### Authentication Operations
- `PUT /user/create` - Register new user
- `POST /user/auth` - Authenticate user request
- `GET /user/:uuid` - Get user information
- `DELETE /user/:uuid` - Delete user

### MAGIC Protocol
- `POST /magic/spell/:spellName` - Execute MAGIC spells for auth operations

### Health & Status
- `GET /health` - Service health check

## MAGIC Route Conversion (October 2025)

All Continuebee REST endpoints have been converted to MAGIC protocol spells:

### Converted Spells (4 total)
1. **continuebeeUserCreate** - Register new user
2. **continuebeeUserAuth** - Authenticate user request
3. **continuebeeUserGet** - Get user information
4. **continuebeeUserDelete** - Delete user

**Testing**: Comprehensive MAGIC spell tests available in `/test/mocha/magic-spells.js` (10 tests covering success and error cases)

**Documentation**: See `/MAGIC-ROUTES.md` for complete spell specifications and migration guide

## Implementation Details

**Location**: `/src/server/node/src/magic/magic.js`

All authentication operations maintain the same functionality as the original REST endpoints while benefiting from centralized Fount authentication and MAGIC protocol features like experience granting and gateway rewards.

**Special Notes**:
- Continuebee is a foundational service used by other services for authentication
- All signature verification logic remains unchanged
- MAGIC spells provide alternative access pattern while maintaining security

## Last Updated
October 14, 2025 - Completed full MAGIC protocol conversion. All 4 routes now accessible via MAGIC spells with centralized Fount authentication.
