#include <Arduino_HTS221.h>

// Just to see that board is alive
const int LED_PIN = 3;

const int MEASUREMENT_TIMEOUT = 4000;
const int BLINKING_TIMEOUT = 1000;

// the setup function runs once when you press reset or power the board
void setup() {
  // Initialize serial (usb)
  Serial.begin(9600);

  // Initialize UART on Serial1
  Serial1.begin(9600);
  
  // initialize pin to light the led
  pinMode(LED_PIN, OUTPUT);

  // initialize HTS library
  if (!HTS.begin()) {
    Serial.println("Failed to initialize humidity temperature sensor!");
    while (1);
  }
}

// the loop function runs over and over again forever
void loop() {
  // Blink the LED to see if stuff works
  digitalWrite(LED_PIN, HIGH);
  delay(BLINKING_TIMEOUT);
  digitalWrite(LED_PIN, LOW);
    
  // Read temperature and write it to serial
  float temperature = HTS.readTemperature();

  Serial.print("Temperature = ");
  Serial.print(temperature);
  Serial.println(" °C");

  // Send "C <temperature>"
  Serial1.print('C');
  Serial1.print(temperature);
  
  // Read humidity and write it to serial
  float humidity = HTS.readHumidity();

  Serial.print("Humidity    = ");
  Serial.print(humidity);
  Serial.println(" %");

  // Give other board some time to read and display measurement
  delay(MEASUREMENT_TIMEOUT);

  // Send "H <humidity>"
  Serial1.print('H');
  Serial1.print(humidity);

  // Give other board some time to read and display measurement
  delay(MEASUREMENT_TIMEOUT);
}
