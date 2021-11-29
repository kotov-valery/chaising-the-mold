#include <LiquidCrystal.h>

// Initialize the library by associating any needed LCD interface pin
// with the arduino pin number it is connected to
const int rs = 12, en = 11, d4 = 5, d5 = 4, d6 = 3, d7 = 2;
LiquidCrystal lcd(11, 12, 5, 4, 3, 2);

// To skip some nonesense during measurements
const long TEMPERATURE_LOWER_BOUND = -50;
const long TEMPERATURE_UPPER_BOUND = 50;

// Timeout between messages
const int MESSAGE_TIMEOUT = 2000;

void setup() {
  // Set up the LCD's number of columns and rows:
  lcd.begin(16, 2);
  // Print welcome message
  lcd.print("How warm is it?");
  // Initialize uart
  Serial.begin(9600);
}

void clear_line(int num) {
  lcd.setCursor(0, num);
  lcd.print("                ");
  lcd.setCursor(0, num);
}

void loop() {
  // Waiting for data...
  clear_line(1);  
  lcd.setCursor(0, 1);
  lcd.print("measuring...");

  // Read data from UART: remperature, humidity
  if (Serial.available() > 0) {
    char c = Serial.read();

    // Read and display temperature
    if (c == 'C') {
      long temperature = Serial.parseInt();

      if (temperature >= TEMPERATURE_LOWER_BOUND && 
            temperature <= TEMPERATURE_UPPER_BOUND) {
        clear_line(1);
        lcd.print("Temp: ");
        lcd.print(temperature);
        lcd.print(" C");
      }
     } else if (c == 'H') {
      // Read and display humidity 
      long humidity = Serial.parseInt();

      clear_line(1);
      lcd.print("Hum: ");
      lcd.print(humidity);
      lcd.print(" %");
    }
  }

  delay(MESSAGE_TIMEOUT);
}
