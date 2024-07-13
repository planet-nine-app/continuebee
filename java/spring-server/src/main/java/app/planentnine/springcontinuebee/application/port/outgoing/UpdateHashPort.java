package app.planentnine.springcontinuebee.application.port.outgoing;

import app.planentnine.springcontinuebee.application.domain.User;

import java.util.UUID;

public interface UpdateHashPort {
    User updateHash(UUID uuid, String newHash);
}
