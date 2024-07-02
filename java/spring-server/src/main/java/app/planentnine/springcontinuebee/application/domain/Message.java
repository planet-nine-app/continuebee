package app.planentnine.springcontinuebee.application.domain;

import java.util.UUID;

public record Message(UUID userUUID, String hash, String payload, String signature, Long timestamp) {
}
