package app.planentnine.springcontinuebee.application.service;

import app.planentnine.springcontinuebee.application.domain.User;
import app.planentnine.springcontinuebee.application.domain.exception.ValidationException;
import app.planentnine.springcontinuebee.application.port.incoming.CreateUserUseCase;
import app.planentnine.springcontinuebee.application.port.outgoing.CreateUserPort;
import app.planentnine.springcontinuebee.application.encryption.Sessionless;
import app.planentnine.springcontinuebee.application.validation.CreateUserValidator;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Optional;
import java.util.UUID;

@Service
public class UserService implements CreateUserUseCase {
    private final CreateUserPort createUserPort;
    private final CreateUserValidator createUserValidator;
    
    @Autowired
    public UserService(CreateUserPort createUserPort, CreateUserValidator createUserValidator){
        this.createUserPort = createUserPort;
        this.createUserValidator = createUserValidator;
    }
    
    @Override
    public User createUser(User user) {
        Optional<ValidationException> validationResult = createUserValidator.validate(user.publicKey());
        if (validationResult.isPresent()) {
            throw validationResult.get();
        }
        
        User createdUser = new User(
                UUID.randomUUID(),
                Sessionless.generateUuid(),
                user.publicKey(),
                user.hash()
        );
        return createUserPort.createUser(createdUser);
    }
}
