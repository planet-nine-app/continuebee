package app.planentnine.springcontinuebee.application.port.incoming;

import app.planentnine.springcontinuebee.application.domain.Message;

public interface UpdateHashUseCase {
    boolean updateHash(Message message, String newHash);
}
