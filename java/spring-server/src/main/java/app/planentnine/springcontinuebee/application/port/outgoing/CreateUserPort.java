package app.planentnine.springcontinuebee.application.port.outgoing;

import app.planentnine.springcontinuebee.application.domain.User;

public interface CreateUserPort {
    User createUser(User user);
}
