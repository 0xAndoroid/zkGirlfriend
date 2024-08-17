# Use an official Python runtime as a parent image
FROM python:3.11-slim

# Set the working directory for Python
WORKDIR /app

# Copy the requirements.txt file into the container at /app
COPY requirements.txt .

# Install any dependencies specified in requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Copy the rest of the application code into the container at /app
COPY . .

# Specify the command to run on container start
CMD ["python", "app.py"]

# If you need to expose any ports, uncomment the following line
# EXPOSE 5000
