package app.planentnine.springcontinuebee.adapter.web.dto;

import lombok.Builder;
import lombok.extern.jackson.Jacksonized;

import java.util.UUID;

@Builder
@Jacksonized
public record RestDeleteUserDto(Long timestamp, UUID userUuid, String hash, String signature) {
}
