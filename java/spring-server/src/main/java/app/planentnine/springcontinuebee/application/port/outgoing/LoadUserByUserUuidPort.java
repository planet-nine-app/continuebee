package app.planentnine.springcontinuebee.application.port.outgoing;

import app.planentnine.springcontinuebee.application.domain.User;

import java.util.Optional;
import java.util.UUID;

public interface LoadUserByUserUuidPort {
    Optional<User> loadByUserUuid(UUID userUuid);
}
