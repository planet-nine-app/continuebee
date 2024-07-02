package app.planentnine.springcontinuebee.adapter.web.dto;

import lombok.Builder;
import lombok.extern.jackson.Jacksonized;

import java.util.UUID;

@Builder
@Jacksonized
public record RestUpdateHashDto(Long timestamp, UUID userUUID, String hash, String newHash, String signature) {
}
