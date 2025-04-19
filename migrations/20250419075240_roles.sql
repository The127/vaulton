-- Create roles table
CREATE TABLE roles
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create user_roles mapping table
CREATE TABLE user_roles
(
    user_id    UUID REFERENCES users(id) ON DELETE CASCADE,
    role_id    UUID REFERENCES roles(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, role_id)
);

-- Create indexes
CREATE INDEX idx_roles_name ON roles (name);
CREATE INDEX idx_user_roles_user_id ON user_roles (user_id);
CREATE INDEX idx_user_roles_role_id ON user_roles (role_id);

-- Triggers for roles table
CREATE TRIGGER set_roles_timestamps
    BEFORE INSERT ON roles
    FOR EACH ROW
EXECUTE FUNCTION set_created_at_column();

CREATE TRIGGER update_roles_updated_at
    BEFORE UPDATE ON roles
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Triggers for user_roles table
CREATE TRIGGER set_user_roles_timestamps
    BEFORE INSERT ON user_roles
    FOR EACH ROW
EXECUTE FUNCTION set_created_at_column();

CREATE TRIGGER update_user_roles_updated_at
    BEFORE UPDATE ON user_roles
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Insert default roles
INSERT INTO roles (name, description)
VALUES ('admin', 'Full system access'),
       ('user', 'Standard user access');