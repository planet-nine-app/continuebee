package app.planentnine.springcontinuebee.application.port.outgoing;

import java.util.UUID;

public interface DeleteUserByUuidPort {
    boolean deleteUserByUuid(UUID userUuid);
}
