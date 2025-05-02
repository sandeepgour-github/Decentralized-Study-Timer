#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Vec, String, log};

// Struct to represent a single study session
#[contracttype]
#[derive(Clone)]
pub struct StudySession {
    pub user: Address,
    pub subject: String,
    pub duration_minutes: u32,
    pub timestamp: u64,
}

// Key for mapping user to their sessions
#[contracttype]
pub enum SessionKey {
    User(Address),
}

#[contract]
pub struct DecentralizedStudyTimer;

#[contractimpl]
impl DecentralizedStudyTimer {
    // Log a new study session
    pub fn log_session(env: Env, user: Address, subject: String, duration_minutes: u32) {
        user.require_auth();

        let mut sessions: Vec<StudySession> =
            env.storage().instance().get(&SessionKey::User(user.clone())).unwrap_or(Vec::new(&env));

        let session = StudySession {
            user: user.clone(),
            subject,
            duration_minutes,
            timestamp: env.ledger().timestamp(),
        };

        sessions.push_back(session);
        env.storage().instance().set(&SessionKey::User(user), &sessions);

        log!(&env, "Study session logged");
    }

    // Retrieve study history for a user
    pub fn get_sessions(env: Env, user: Address) -> Vec<StudySession> {
        env.storage().instance().get(&SessionKey::User(user)).unwrap_or(Vec::new(&env))
    }

    // Get total study time in minutes
    pub fn get_total_study_time(env: Env, user: Address) -> u32 {
        let sessions = Self::get_sessions(env.clone(), user);
        sessions.iter().fold(0, |sum, session| sum + session.duration_minutes)
    }
}
