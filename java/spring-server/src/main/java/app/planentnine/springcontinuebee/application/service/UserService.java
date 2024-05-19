package app.planentnine.springcontinuebee.application.service;

import app.planentnine.springcontinuebee.application.domain.User;
import app.planentnine.springcontinuebee.application.domain.exception.ValidationException;
import app.planentnine.springcontinuebee.application.port.incoming.CreateUserUseCase;
import app.planentnine.springcontinuebee.application.port.incoming.DeleteUserUseCase;
import app.planentnine.springcontinuebee.application.port.outgoing.CreateUserIfNotExistsPort;
import app.planentnine.springcontinuebee.application.encryption.Sessionless;
import app.planentnine.springcontinuebee.application.port.outgoing.DeleteUserByUuidPort;
import app.planentnine.springcontinuebee.application.validation.CreateUserValidator;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Optional;
import java.util.UUID;

@Service
public class UserService implements CreateUserUseCase, DeleteUserUseCase {
    private final CreateUserIfNotExistsPort createUserIfNotExistsPort;
    private final DeleteUserByUuidPort deleteUserByUuidPort;
    private final CreateUserValidator createUserValidator;
    
    @Autowired
    public UserService(CreateUserIfNotExistsPort createUserIfNotExistsPort, DeleteUserByUuidPort deleteUserByUuidPort, CreateUserValidator createUserValidator){
        this.createUserIfNotExistsPort = createUserIfNotExistsPort;
        this.deleteUserByUuidPort = deleteUserByUuidPort;
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
        
        return createUserIfNotExistsPort.createUserIfNotExists(createdUser);
    }
    
    @Override
    public boolean deleteUser(UUID uuid) {
        
        return deleteUserByUuidPort.deleteUserByUuid(uuid);
    }
}
