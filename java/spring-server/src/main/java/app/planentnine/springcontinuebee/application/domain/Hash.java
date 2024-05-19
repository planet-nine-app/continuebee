package app.planentnine.springcontinuebee.application.domain;

import java.time.LocalDateTime;
import java.util.UUID;

public record Hash(UUID userUuid, String hashString, String[] signature, LocalDateTime timestamp) {
}
