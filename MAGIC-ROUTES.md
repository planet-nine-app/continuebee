# Continuebee MAGIC-Routed Endpoints

## Overview

Continuebee now supports MAGIC-routed versions of all POST, PUT, and DELETE operations. These spells route through Fount (the resolver) for centralized authentication, eliminating the need for direct signature verification in Continuebee.

## Converted Routes

### 1. Create User
**Direct Route**: `POST /user/create`
**MAGIC Spell**: `continuebeeUserCreate`
**Cost**: 50 MP

**Components**:
```javascript
{
  pubKey: "user-public-key",
  hash: "password-hash"
}
```

**Returns**:
```javascript
{
  success: true,
  user: {
    uuid: "user-uuid",
    pubKey: "user-public-key",
    hash: "password-hash"
  }
}
```

**Validation**:
- Requires pubKey and hash
- Returns existing user if pubKey and hash match
- Creates new user if not exists

**Hash Format**:
The hash is a SHA-256 digest of `email:password`:
```javascript
import { createHash } from 'crypto';
const hash = createHash('sha256');
hash.update('user@example.com:PASSWORD123');
const passwordHash = hash.digest('hex');
```

---

### 2. Update User Hash
**Direct Route**: `PUT /user/update-hash`
**MAGIC Spell**: `continuebeeUserUpdateHash`
**Cost**: 50 MP

**Components**:
```javascript
{
  userUUID: "user-uuid",
  hash: "current-password-hash",
  newHash: "new-password-hash"
}
```

**Returns**:
```javascript
{
  success: true,
  user: {
    uuid: "user-uuid",
    pubKey: "user-public-key",
    hash: "new-password-hash"
  }
}
```

**Validation**:
- Requires userUUID, hash, and newHash
- Verifies current hash matches before updating
- Returns error if current hash doesn't match

---

### 3. Delete User
**Direct Route**: `DELETE /user/delete`
**MAGIC Spell**: `continuebeeUserDelete`
**Cost**: 50 MP

**Components**:
```javascript
{
  userUUID: "user-uuid",
  hash: "password-hash"
}
```

**Returns**:
```javascript
{
  success: true
}
```

**Validation**:
- Requires userUUID and hash
- Verifies hash matches before deletion
- Returns error if hash doesn't match

---

## Implementation Details

### File Changes

1. **`/src/server/node/src/magic/magic.js`** - Added three new spell handlers:
   - `continuebeeUserCreate(spell)`
   - `continuebeeUserUpdateHash(spell)`
   - `continuebeeUserDelete(spell)`

2. **`/fount/src/server/node/spellbooks/spellbook.js`** - Added spell definitions with destinations and costs

3. **`/test/mocha/magic-spells.js`** - New test file with comprehensive spell tests

4. **`/test/mocha/package.json`** - Added `fount-js` dependency

### Authentication Flow

```
Client → Fount (resolver) → Continuebee MAGIC handler → Business logic
           ↓
    Verifies signature
    Deducts MP
    Grants experience
    Grants nineum
```

**Before (Direct REST)**:
- Client signs request with message: `timestamp + pubKey + hash`
- Continuebee verifies signature directly
- Continuebee executes business logic

**After (MAGIC Spell)**:
- Client signs spell
- Fount verifies signature & deducts MP
- Fount grants experience & nineum to caster
- Fount forwards to Continuebee
- Continuebee executes business logic (no auth needed)

### Naming Convention

Route path → Spell name transformation:
```
/user/create         → continuebeeUserCreate
/user/update-hash    → continuebeeUserUpdateHash
/user/delete         → continuebeeUserDelete
```

Pattern: `[service][PathWithoutSlashesAndParams]`

### Password Hash System

Continuebee uses SHA-256 hashes of `email:password` for authentication:

```javascript
// Creating a password hash
import { createHash } from 'crypto';

const email = 'user@example.com';
const password = 'MySecurePassword123';

const hash = createHash('sha256');
hash.update(`${email}:${password}`);
const passwordHash = hash.digest('hex');
// passwordHash is stored and used for authentication
```

**Why Email:Password Format?**
- Ties authentication to specific email address
- Prevents rainbow table attacks across services
- Changing email requires password update

**Security Notes**:
- Hash is computed client-side, never send plain password
- Server never sees or stores plain password
- Changing password creates entirely new hash
- Hash verification is simple equality check

### Error Handling

All spell handlers return consistent error format:
```javascript
{
  success: false,
  error: "Error description"
}
```

**Common Errors**:
- Missing required fields (pubKey, hash, userUUID, newHash)
- User not found
- Hash mismatch (current hash doesn't match)
- User already exists (different hash for same pubKey)

## Testing

Run MAGIC spell tests:
```bash
cd continuebee/test/mocha
npm install
npm test magic-spells.js
```

Test coverage:
- ✅ User creation via spell
- ✅ Existing user return via spell
- ✅ Hash update via spell
- ✅ User deletion via spell
- ✅ Missing pubKey validation
- ✅ Missing hash validation
- ✅ Wrong current hash validation
- ✅ Missing fields validation
- ✅ Missing newHash validation

## Benefits

1. **No Direct Authentication**: Continuebee handlers don't need to verify signatures
2. **Centralized Auth**: All signature verification in one place (Fount)
3. **Automatic Rewards**: Every spell grants experience + nineum
4. **Gateway Rewards**: Gateway participants get 10% of rewards
5. **Reduced Code**: Continuebee handlers simplified without auth logic
6. **Consistent Pattern**: Same flow across all services

## Differences from Direct REST

### Authentication
- **REST**: Direct signature verification in Continuebee using `sessionless.verifySignature()`
- **MAGIC**: Fount verifies, Continuebee trusts resolver

### Message Format
- **REST**: `timestamp + pubKey + hash` (create), `timestamp + uuid + hash + newHash` (update), etc.
- **MAGIC**: Fount handles signature, spell components contain just business data

### User Lookup
Both REST and MAGIC use the same internal methods:
- `user.putUser()` - Creates new user
- `user.getUser()` - Retrieves user by UUID
- `user.updateHash()` - Updates user's password hash
- `user.deleteUser()` - Deletes user

## Use Cases

### Password Management Service
Continuebee serves as the core authentication service for Planet Nine:

1. **User Registration**: Create account with email-derived hash
2. **Password Change**: Update hash when user changes password
3. **Account Deletion**: Remove user and all authentication data
4. **Login Verification**: Check if provided hash matches stored hash

### Integration with Other Services
Other services can verify users through Continuebee:
- Joan uses continuebee for user authentication
- Pref uses continuebee for preference access control
- All services can leverage centralized password management

## Next Steps

Progress on MAGIC route conversion:
- ✅ Joan (3 routes complete)
- ✅ Pref (4 routes complete)
- ✅ Aretha (4 routes complete)
- ✅ Continuebee (3 routes complete)
- ⏳ BDO
- ⏳ Julia
- ⏳ Dolores
- ⏳ Sanora
- ⏳ Addie
- ⏳ Covenant
- ⏳ Prof
- ⏳ Fount (internal routes)
- ⏳ Minnie (SMTP only, no HTTP routes)

## Last Updated
January 14, 2025
