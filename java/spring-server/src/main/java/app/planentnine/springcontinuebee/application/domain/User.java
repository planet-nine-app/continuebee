package app.planentnine.springcontinuebee.application.domain;

import java.util.UUID;

public record User(UUID id, UUID userUuid, String publicKey, String hash) {
}
