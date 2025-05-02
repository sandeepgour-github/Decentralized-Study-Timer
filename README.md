# Decentralized Study Timer

## 📚 Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

---

## 📌 Project Title
**Decentralized Study Timer**

## 📝 Project Description
A blockchain-powered tool that allows students to track and record their study sessions on-chain. By decentralizing study data, students can own and verify their productivity over time with complete transparency and privacy.

## 🌍 Project Vision
To create a transparent and tamper-proof academic productivity tool that helps students track learning patterns, stay accountable, and get rewarded for consistent effort.

## ✨ Key Features
- Log study sessions with subject and duration.
- View detailed study session history.
- Calculate total study time per user.
- All data is stored immutably on-chain and tied to the user's address.

## 🔍 Contract Details

### Contract Address: CCDWRA5UG6EAIBH3PGNFKX62UM7V4GIRLBQYV6YHA5QXFP7E3ZP3FJPG

### `log_session`
- Records a new study session.
- Parameters:
  - `user: Address`
  - `subject: String`
  - `duration_minutes: u32`

### `get_sessions`
- Retrieves the list of study sessions for a specific user.
- Parameter:
  - `user: Address`
- Returns:
  - `Vec<StudySession>`

### `get_total_study_time`
- Returns the total number of minutes studied by the user.
- Parameter:
  - `user: Address`
- Returns:
  - `u32`

---

*Built using [Soroban SDK](https://soroban.stellar.org/docs)* 🧠💻 
![Screenshot (88)](https://github.com/user-attachments/assets/0441ba86-3005-440d-bd0b-afc9c48dda0c)
