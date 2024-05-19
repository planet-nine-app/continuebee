package app.planentnine.springcontinuebee.application.port.incoming;

import java.util.UUID;

public interface DeleteUserUseCase {
    boolean deleteUser(UUID userUuid);
}
