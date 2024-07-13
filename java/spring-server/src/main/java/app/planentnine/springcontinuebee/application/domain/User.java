package app.planentnine.springcontinuebee.application.domain;

import java.util.UUID;

public record User(UUID id, UUID userUUID, String publicKey, String hash) {
}
