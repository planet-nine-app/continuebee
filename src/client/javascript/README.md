# Continuebee

This is the JavaScript client SDK for the Continuebee miniservice. 

### Usage

```javascript
import continuebee from 'continuebee-js';

const saveKeys = (keys) => { /* handle persisting keys here */ };
const getKeys = () => { /* return keys here. Can be async */ };

const stateHash = 'add a hash of the current state';

const uuid = await continuebee.createUser(stateHash, saveKeys, getKeys);

const isExpectedState = await continuebee.checkHash(uuid, stateHash); // returns true if stateHash is valid

const newStateHash = 'when state changes, update the hash';

const updated = await continuebee.updateHash(uuid, stateHash, newStateHash); // returns true on success

const deleted = await continuebee.deleteUser(uuid, newStateHash); // returns true on success
```
