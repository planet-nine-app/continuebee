package app.planentnine.springcontinuebee.adapter.persistence;

import app.planentnine.springcontinuebee.adapter.persistence.entity.mapper.PostgresUserEntityMapper;
import app.planentnine.springcontinuebee.adapter.persistence.mybatis.PostgresUserRepository;
import app.planentnine.springcontinuebee.application.domain.User;
import app.planentnine.springcontinuebee.application.port.outgoing.CreateUserIfNotExistsPort;
import app.planentnine.springcontinuebee.application.port.outgoing.DeleteUserByUuidPort;
import app.planentnine.springcontinuebee.application.port.outgoing.UpdateHashPort;
import app.planentnine.springcontinuebee.application.port.outgoing.LoadUserByUserUuidPort;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Repository;

import java.util.Optional;
import java.util.UUID;

@Repository
@Slf4j
public class UserRepository implements CreateUserIfNotExistsPort, DeleteUserByUuidPort, UpdateHashPort, LoadUserByUserUuidPort {
    
    private final PostgresUserRepository postgresUserRepository;
    private final PostgresUserEntityMapper postgresUserEntityMapper;
    
    @Autowired
    public UserRepository(PostgresUserRepository postgresUserRepository,
                          PostgresUserEntityMapper postgresUserEntityMapper){
        this.postgresUserRepository = postgresUserRepository;
        this.postgresUserEntityMapper = postgresUserEntityMapper;
    }
    
    @Override
    public boolean deleteUserByUuid(UUID userUuid) {
        postgresUserRepository.deleteUserByUuid(userUuid);
        
        return loadByUserUuid(userUuid).isEmpty();
    }
    
    
    @Override
    public User updateHash(UUID uuid, String newHash) {
        postgresUserRepository.updateHash(uuid, newHash);
        
        return loadByUserUuid(uuid)
                .orElseThrow(() -> new RuntimeException("Something went wrong inserting hash for user: " + uuid));
    }
    
    @Override
    public User createUserIfNotExists(User user) {
        postgresUserRepository.createNewUser(postgresUserEntityMapper.map(user));
        return loadByUserUuid(user.userUuid())
                .orElseThrow(() -> new RuntimeException("Something went wrong creating new user: " + user.userUuid()));
    }
    
    @Override
    public Optional<User> loadByUserUuid(UUID userUuid) {
        return postgresUserRepository.loadUserByUuid(userUuid).map(postgresUserEntityMapper::map);
    }
    
}
