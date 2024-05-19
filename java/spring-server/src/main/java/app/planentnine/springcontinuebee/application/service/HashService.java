package app.planentnine.springcontinuebee.application.service;

import app.planentnine.springcontinuebee.application.domain.Hash;
import app.planentnine.springcontinuebee.application.domain.exception.ValidationException;
import app.planentnine.springcontinuebee.application.encryption.Sessionless;
import app.planentnine.springcontinuebee.application.domain.User;
import app.planentnine.springcontinuebee.application.port.incoming.VerifyHashUseCase;
import app.planentnine.springcontinuebee.application.port.outgoing.LoadUserByUserUuidPort;
import app.planentnine.springcontinuebee.application.validation.VerifyHashValidator;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Optional;

@Service
public class HashService implements VerifyHashUseCase {
    private final LoadUserByUserUuidPort loadUserByUserUuidPort;
    private final VerifyHashValidator verifyHashValidator;
    
    @Autowired
    public HashService(LoadUserByUserUuidPort loadUserByUserUuidPort, VerifyHashValidator verifyHashValidator) {
        this.loadUserByUserUuidPort = loadUserByUserUuidPort;
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
        return Sessionless.verifySignature(publicKey, signature, hashString);
    }
}
