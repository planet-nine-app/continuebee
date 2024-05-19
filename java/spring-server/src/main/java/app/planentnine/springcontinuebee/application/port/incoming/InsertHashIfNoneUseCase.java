package app.planentnine.springcontinuebee.application.port.incoming;

import java.util.UUID;

public interface InsertHashIfNoneUseCase {
    boolean insertHashIfNone(UUID uuid, String hash);
}
