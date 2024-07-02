package app.planentnine.springcontinuebee.adapter.persistence.entity.mapper;

import app.planentnine.springcontinuebee.adapter.persistence.entity.PostgresUserEntity;
import app.planentnine.springcontinuebee.application.domain.User;
import org.springframework.stereotype.Component;

@Component
public class PostgresUserEntityMapper {
    public User map(PostgresUserEntity postgresUserEntity){
        return new User(
                postgresUserEntity.getId(),
                postgresUserEntity.getUserUUID(),
                postgresUserEntity.getPublicKey(),
                postgresUserEntity.getHash()
        );
    }
    
    public PostgresUserEntity map(User user){
        return PostgresUserEntity.builder()
                .id(user.id())
                .userUUID(user.userUUID())
                .publicKey(user.publicKey())
                .hash(user.hash())
                .build();
    }
}
