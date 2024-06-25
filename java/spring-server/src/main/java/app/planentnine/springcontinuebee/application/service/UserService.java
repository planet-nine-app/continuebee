package app.planentnine.springcontinuebee.application.service;

import app.planentnine.springcontinuebee.application.domain.Message;
import app.planentnine.springcontinuebee.application.domain.User;
import app.planentnine.springcontinuebee.application.domain.exception.ValidationException;
import app.planentnine.springcontinuebee.application.port.incoming.CreateUserUseCase;
import app.planentnine.springcontinuebee.application.port.incoming.DeleteUserUseCase;
import app.planentnine.springcontinuebee.application.port.incoming.UpdateHashUseCase;
import app.planentnine.springcontinuebee.application.port.incoming.VerifyHashUseCase;
import app.planentnine.springcontinuebee.application.port.outgoing.CreateUserIfNotExistsPort;
import app.planentnine.springcontinuebee.application.encryption.Sessionless;
import app.planentnine.springcontinuebee.application.port.outgoing.DeleteUserByUuidPort;
import app.planentnine.springcontinuebee.application.port.outgoing.LoadUserByUserUuidPort;
import app.planentnine.springcontinuebee.application.port.outgoing.UpdateHashPort;
import app.planentnine.springcontinuebee.application.validation.CreateUserValidator;
import app.planentnine.springcontinuebee.application.validation.MessageFormatValidator;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.Optional;
import java.util.UUID;

@Service
public class UserService implements CreateUserUseCase, DeleteUserUseCase, VerifyHashUseCase, UpdateHashUseCase{
    private final CreateUserIfNotExistsPort createUserIfNotExistsPort;
    private final DeleteUserByUuidPort deleteUserByUuidPort;
    private final CreateUserValidator createUserValidator;
    private final LoadUserByUserUuidPort loadUserByUserUuidPort;
    private final UpdateHashPort updateHashPort;
    private final MessageFormatValidator messageFormatValidator;
    
    @Autowired
    public UserService(CreateUserIfNotExistsPort createUserIfNotExistsPort, DeleteUserByUuidPort deleteUserByUuidPort, CreateUserValidator createUserValidator,
                       LoadUserByUserUuidPort loadUserByUserUuidPort, UpdateHashPort updateHashPort, MessageFormatValidator messageFormatValidator){
        this.createUserIfNotExistsPort = createUserIfNotExistsPort;
        this.deleteUserByUuidPort = deleteUserByUuidPort;
        this.createUserValidator = createUserValidator;
        this.loadUserByUserUuidPort = loadUserByUserUuidPort;
        this.updateHashPort = updateHashPort;
        this.messageFormatValidator = messageFormatValidator;
    }
    
    @Override
    public User createUser(Message message, User user) {
        Optional<ValidationException> validationResult = createUserValidator.validate(user.publicKey());
        if (validationResult.isPresent()) {
            throw validationResult.get();
        }
        if (verifyHash(message)) {
            User createdUser = new User(
                    UUID.randomUUID(),
                    Sessionless.generateUuid(),
                    user.publicKey(),
                    user.hash()
            );
            
            return createUserIfNotExistsPort.createUserIfNotExists(createdUser);
        } else {
            return null;
        }
    }
    
    @Override
    public boolean updateHash(Message message, String newHash) {
        if (verifyHash(message)) {
            User user = updateHashPort.updateHash(message.userUuid(), newHash);
            return user.hash().equals(newHash);
        } else {
            return false;
        }
    }
    
    @Override
    public boolean deleteUser(Message message) {
        if (verifyHash(message)) {
            return deleteUserByUuidPort.deleteUserByUuid(message.userUuid());
        } else {
            return false;
        }
    }
    
    
    @Override
    public boolean verifyHash(Message message) {
        return validateMessage(message);
    }
    
    
    // Get users public key, check the message paylood using sessionles with PubKey + payload + signature,
    // verify timestamp within window, Confirm hash matches
    public boolean validateMessage(Message message) {
        User user = loadUserByUserUuidPort.loadByUserUuid(message.userUuid())
                .orElseThrow(() -> new RuntimeException("User with id: " + message.userUuid() + "could not be found from message"));
        
        Optional<ValidationException> validationResult =
                messageFormatValidator.validate(user.publicKey(), message);
        if (validationResult.isPresent()) {
            throw validationResult.get();
        } else {
            return user.hash().equals(message.hash());
        }
    }
}
