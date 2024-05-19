package app.planentnine.springcontinuebee.adapter.web.dto;

import lombok.Builder;
import lombok.extern.jackson.Jacksonized;

import java.time.LocalDateTime;
import java.util.UUID;

@Builder
@Jacksonized
public record RestHashDto(UUID uuid, LocalDateTime timestamp, String hash, String[] signature) {
}
