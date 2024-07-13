package app.planentnine.springcontinuebee.adapter.web.dto;

import java.util.UUID;

public record RestVerifyHashDto(Long timestamp, UUID userUUID, String hash, String signature) {
}
