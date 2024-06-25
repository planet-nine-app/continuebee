package app.planentnine.springcontinuebee.application.domain;

import java.util.UUID;

public record Message(UUID userUuid, String hash, String payload, String signature, Long timestamp) {
}
