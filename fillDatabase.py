import requests
import random

# Function to generate random users
def generate_user():
    names = ["Alice", "Bob", "Charlie", "David", "Emma", "Frank", "Grace", "Henry", "Isabella", "Jack",
             "Liam", "Olivia", "Noah", "Ava", "William", "Sophia", "James", "Charlotte", "Benjamin", "Amelia",
             "Lucas", "Evelyn", "Alexander", "Abigail", "Michael", "Harper", "Ethan", "Emily", "Daniel", "Elizabeth",
             "Matthew", "Mia", "Jackson", "Ella", "David", "Victoria", "Joseph", "Penelope", "Samuel", "Avery",
             "Sebastian", "Scarlett", "John", "Grace", "Gabriel", "Chloe", "Anthony", "Sofia", "Dylan", "Lily",
             "Christopher", "Luna", "Joshua", "Hannah", "Andrew", "Nora", "Logan", "Addison", "Luke", "Mila",
             "Ryan", "Leah", "Nathan", "Zoey", "Carter", "Audrey", "Christian", "Bella", "Isaac", "Aria",
             "Owen", "Lucy", "Henry", "Genesis", "Wyatt", "Stella", "Caleb", "Violet", "Elijah", "Claire",
             "Levi", "Anna", "Muhammad", "Sadie", "Connor", "Taylor", "Eli", "Alexa", "Aaron", "Kylie"]

    locations = ["Berlin", "Hamburg", "Munich", "Cologne", "Frankfurt", "Stuttgart", "Düsseldorf", "Dortmund", "Essen", "Leipzig",
                 "Hannover", "Nuremberg", "Duisburg", "Bochum", "Wuppertal", "Bielefeld", "Bonn", "Münster", "Karlsruhe", "Mannheim",
                 "Augsburg", "Wiesbaden", "Gelsenkirchen", "Mönchengladbach", "Braunschweig", "Chemnitz", "Kiel", "Aachen", "Halle", "Magdeburg",
                 "Freiburg", "Krefeld", "Lübeck", "Oberhausen", "Erfurt", "Mainz", "Rostock", "Kassel", "Hagen", "Hamm",
                 "Saarbrücken", "Mülheim an der Ruhr", "Potsdam", "Ludwigshafen", "Oldenburg", "Leverkusen", "Osnabrück", "Solingen", "Heidelberg",
                 "Herne", "Neuss", "Darmstadt", "Paderborn", "Regensburg", "Ingolstadt", "Würzburg", "Fürth", "Ulm", "Heilbronn"]

    titles = ["Student", "Engineer", "Doctor", "Teacher", "Artist", "Developer", "Designer", "Manager", "Chef", "Nurse",
              "Scientist", "Accountant", "Writer", "Lawyer", "Consultant", "Entrepreneur", "Architect", "Analyst", "Therapist", "Pharmacist",
              "Programmer", "Researcher", "Musician", "Veterinarian", "Marketing Manager", "Sales Representative", "Executive Assistant", "Financial Analyst", "Social Worker", "Electrician",
              "Plumber", "Mechanic", "Paramedic", "Police Officer", "Firefighter", "Pilot", "Flight Attendant", "Journalist", "Librarian", "Translator",
              "Photographer", "Psychologist", "Dentist", "Graphic Designer", "Interior Designer", "Fitness Trainer", "Nutritionist", "Singer", "Actor",
              "Director", "Producer", "Dancer", "Model", "Athlete", "Coach", "Biologist", "Chemist", "Geologist", "Physicist"]

    name = random.choice(names)
    location = random.choice(locations)
    title = random.choice(titles)

    return {
        "name": name,
        "location": location,
        "title": title
    }

# URL of the API endpoint
url = "http://127.0.0.1:8000/user/add"

# Number of users to generate
num_users = 1000

# Loop to generate and add users
for _ in range(num_users):
    user_data = generate_user()
    response = requests.post(url, json=user_data)
    
    if response.status_code == 200:
        print("User added successfully:", user_data)
    else:
        print("Failed to add user:", user_data)
        print("Response:", response.text)

