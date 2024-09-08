console.log(window.konnektoren);

const data = window.konnektoren ? window.konnektoren.challenge.data :
    new Map([
        ["options", [
            new Map([["id", 0], ["name", "der"]]),
            new Map([["id", 1], ["name", "die"]]),
            new Map([["id", 2], ["name", "das"]])
        ]],
        ["questions", [
            new Map([
                ["question", "What is the article for 'Haus'?"],
                ["option", 2], // 'das' is correct
                ["help", "The article for 'Haus' is 'das'."]
            ]),
            new Map([
                ["question", "What is the article for 'Tisch'?"],
                ["option", 0], // 'der' is correct
                ["help", "The article for 'Tisch' is 'der'."]
            ]),
            new Map([
                ["question", "What is the article for 'Buch'?"],
                ["option", 2], // 'das' is correct
                ["help", "The article for 'Buch' is 'das'."]
            ]),
        ]]
    ]);

console.log(data);

let currentQuestionIndex = 0;

function finishChallenge() {
    if (window.konnektoren.sendEvent) {
        const event = { type: "Finish", result: {} };
        window.konnektoren.sendEvent(event);
    }
}

// Load the current question
function loadQuestion() {

    const currentQuestion = data.get("questions")[currentQuestionIndex];
    const questionText = document.getElementById('question-text');
    const helpText = document.getElementById('help-text');
    const questionImage = document.getElementById('question-image');
    const optionsList = document.getElementById('options-list');
    const feedback = document.getElementById('feedback');

    // Reset feedback on new question
    feedback.textContent = '';
    feedback.style.display = 'none';

    // Update question and help text
    questionText.textContent = currentQuestion.get("question");
    helpText.textContent = currentQuestion.get("help");

    // Update question image if available
    if (currentQuestion.get("image")) {
        questionImage.className = "fas " + currentQuestion.get("image");
    } else {
        questionImage.className = "";
    }

    // Clear previous options
    optionsList.innerHTML = '';

    // Create and display options
    data.get("options").forEach(option => {
        const li = document.createElement('li');
        li.textContent = option.get("name");
        li.dataset.id = option.get("id");

        // Handle option click
        li.addEventListener('click', () => {
            checkAnswer(option.get("id"));
        });

        optionsList.appendChild(li);
    });
}

function nextQuestion() {
    currentQuestionIndex++;
    loadQuestion();
}

// Check if the selected answer is correct
function checkAnswer(selectedOption) {
    const currentQuestion = data.get("questions")[currentQuestionIndex];
    const feedback = document.getElementById('feedback');

    // Show feedback based on whether the answer is correct or not
    if (selectedOption === currentQuestion.get("option")) {
        feedback.textContent = 'Correct!';
        feedback.style.color = 'green';
    } else {
        feedback.textContent = 'Incorrect, try again!';
        feedback.style.color = 'red';
    }

    feedback.style.display = 'block';

    if(currentQuestionIndex === data.get("questions").length - 1) {
        finishChallenge();
        return;
    } else {
        setTimeout(() => {
            nextQuestion();
        }, 1000);
    }
}

// Automatically load the first question on page load
loadQuestion();

document.getElementById("finish-button").addEventListener("click", function () {
    finishChallenge();
});