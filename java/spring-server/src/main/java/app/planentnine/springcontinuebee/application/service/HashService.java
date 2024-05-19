package app.planentnine.springcontinuebee.application.service;

import app.planentnine.springcontinuebee.application.domain.Hash;
import app.planentnine.springcontinuebee.application.domain.exception.ValidationException;
import app.planentnine.springcontinuebee.application.encryption.Sessionless;
import app.planentnine.springcontinuebee.application.domain.User;
import app.planentnine.springcontinuebee.application.port.incoming.InsertHashIfNoneUseCase;
import app.planentnine.springcontinuebee.application.port.incoming.VerifyHashUseCase;
import app.planentnine.springcontinuebee.application.port.outgoing.InsertHashIfNonePort;
import app.planentnine.springcontinuebee.application.port.outgoing.LoadUserByUserUuidPort;
import app.planentnine.springcontinuebee.application.validation.VerifyHashValidator;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Optional;
import java.util.UUID;

@Service
public class HashService implements VerifyHashUseCase, InsertHashIfNoneUseCase {
    private final LoadUserByUserUuidPort loadUserByUserUuidPort;
    private final InsertHashIfNonePort insertHashIfNonePort;
    private final VerifyHashValidator verifyHashValidator;
    
    @Autowired
    public HashService(LoadUserByUserUuidPort loadUserByUserUuidPort, InsertHashIfNonePort insertHashIfNonePort, VerifyHashValidator verifyHashValidator) {
        this.loadUserByUserUuidPort = loadUserByUserUuidPort;
        this.insertHashIfNonePort = insertHashIfNonePort;
        this.verifyHashValidator = verifyHashValidator;
    }
    
    @Override
    public boolean verifyHash(Hash hash) {
        Optional<ValidationException> validationResult = verifyHashValidator.validate(hash.timestamp());
        if (validationResult.isPresent()) {
            throw validationResult.get();
        }
        
        User user = loadUserByUserUuidPort.loadByUserUuid(hash.userUuid())
                .orElseThrow(() -> new RuntimeException("User with id: " + hash.userUuid() + "could not be found from hash"));
        String publicKey = user.publicKey();
        String[] signature = hash.signature();
        String hashString = hash.hashString();
        
        return Sessionless.verifySignature(publicKey, signature, hashString) && user.hash().equals(hash.hashString());
    }
    
    @Override
    public boolean insertHashIfNone(UUID uuid, String hash) {
        
        User user = loadUserByUserUuidPort.loadByUserUuid(uuid)
                .orElseThrow(() -> new RuntimeException("User with id: " + uuid + "could not be found from hash"));
        
        if (user.hash() != null){
            return false;
        }
        
        return insertHashIfNonePort.insertHashIfNone(uuid, hash).hash() != null;
    }
}
