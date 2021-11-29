/**
 * @brief Display temperature on a screen
 * @details The prototype which reads humidity and temperature from the DHT22 sensor and
 *  outputs the result on SSD1306 128x64 OLED screen via I2C
 * Was inspired by the following resources:
 *  - https://randomnerdtutorials.com/guide-for-oled-display-with-arduino/
 *  - https://microcontrollerslab.com/dht11-dht22-arduino-display-readings-oled/
 *  - https://github.com/adafruit/Adafruit_SSD1306/blob/master/examples/ssd1306_128x64_i2c/ssd1306_128x64_i2c.ino#L29
 *  - https://github.com/adafruit/DHT-sensor-library/blob/master/examples/DHTtester/DHTtester.ino
 * Requires the following libraries:
 *  - DHT Sensor Library: https://github.com/adafruit/DHT-sensor-library
 *  - Adafruit Unified Sensor Library: https://github.com/adafruit/Adafruit_Sensor
 *  - Adafruit BusIO Library: https://github.com/adafruit/Adafruit_BusIO
 *  - Adafruit GFX Library: https://github.com/adafruit/Adafruit-GFX-Library
 */

#include <Adafruit_Sensor.h>
#include <DHT.h>
#include <DHT_U.h>

#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>

// Common constants
constexpr const unsigned long SERIAL_SPEED = 115200;

// Timing values
unsigned long MEASUREMENTS_DELAY = 5000;
constexpr const unsigned long SHOW_DATA_DELAY = 5000;

//
// DHT temperature and humidity sensor
//
constexpr const uint8_t DHT_DATA_PIN = 2;
constexpr const uint8_t DHT_SENSOR_TYPE = DHT22;
DHT_Unified dht(DHT_DATA_PIN, DHT_SENSOR_TYPE);

//
// SSD1306 OLED Screen
//
// Declaration for an SSD1306 display connected to I2C (SDA, SCL pins)
// The pins for I2C are defined by the Wire-library.
// On an arduino UNO:       A4(SDA), A5(SCL)
// On an arduino MEGA 2560: 20(SDA), 21(SCL)
// On an arduino LEONARDO:   2(SDA),  3(SCL), ...
constexpr const uint8_t SCREEN_WIDTH = 128;
constexpr const uint8_t SCREEN_HEIGHT = 64;
constexpr const int8_t OLED_RESET = 4;
constexpr const uint8_t SCREEN_ADDRESS = 0x3C;
Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RESET);

void setup() {
  Serial.begin(SERIAL_SPEED);

  // SSD1306_SWITCHCAPVCC = generate display voltage from 3.3V internally
  Serial.println(F("SSD1306 OLED Screen Initialization"));
  if(!display.begin(SSD1306_SWITCHCAPVCC, SCREEN_ADDRESS)) {
    Serial.println(F("SSD1306 allocation failed"));
    for(;;); // Don't proceed, loop forever
  }

  // Show initial display buffer contents on the screen --
  // the library initializes this with an Adafruit splash screen.
  display.display();

  // Initialize DHT sensor
  dht.begin();
  Serial.println(F("DHT22 Sensor Initialization"));

  // Print temperature sensor details.
  sensor_t sensor;
  dht.temperature().getSensor(&sensor);
  Serial.println(F("------------------------------------"));
  Serial.println(F("Temperature Sensor"));
  Serial.print  (F("Sensor Type: ")); Serial.println(sensor.name);
  Serial.print  (F("Driver Ver:  ")); Serial.println(sensor.version);
  Serial.print  (F("Unique ID:   ")); Serial.println(sensor.sensor_id);
  Serial.print  (F("Max Value:   ")); Serial.print(sensor.max_value); Serial.println(F("°C"));
  Serial.print  (F("Min Value:   ")); Serial.print(sensor.min_value); Serial.println(F("°C"));
  Serial.print  (F("Resolution:  ")); Serial.print(sensor.resolution); Serial.println(F("°C"));
  Serial.println(F("------------------------------------"));

  // Print humidity sensor details.
  dht.humidity().getSensor(&sensor);
  Serial.println(F("Humidity Sensor"));
  Serial.print  (F("Sensor Type: ")); Serial.println(sensor.name);
  Serial.print  (F("Driver Ver:  ")); Serial.println(sensor.version);
  Serial.print  (F("Unique ID:   ")); Serial.println(sensor.sensor_id);
  Serial.print  (F("Max Value:   ")); Serial.print(sensor.max_value); Serial.println(F("%"));
  Serial.print  (F("Min Value:   ")); Serial.print(sensor.min_value); Serial.println(F("%"));
  Serial.print  (F("Resolution:  ")); Serial.print(sensor.resolution); Serial.println(F("%"));
  Serial.println(F("------------------------------------"));

  // Set delay between sensor readings based on sensor details.
  MEASUREMENTS_DELAY = sensor.min_delay / 1000;
}

void loop() {
  //
  // Measure temperature and humidity
  //

  // Delay between measurements.
  delay(MEASUREMENTS_DELAY);

  // Get temperature event and print its value.
  sensors_event_t event {};
  float temperature {};
  dht.temperature().getEvent(&event);
  if (isnan(event.temperature)) {
    temperature = 0.0;
    Serial.println(F("Error reading temperature!"));
  } else {
    temperature = event.temperature;
    Serial.print(F("Temperature: "));
    Serial.print(event.temperature);
    Serial.println(F("°C"));
  }

  // Get humidity event and print its value.
  float humidity {};
  dht.humidity().getEvent(&event);
  if (isnan(event.relative_humidity)) {
    humidity = 0.0;
    Serial.println(F("Error reading humidity!"));
  } else {
    humidity = event.relative_humidity;
    Serial.print(F("Humidity: "));
    Serial.print(event.relative_humidity);
    Serial.println(F("%"));
  }

  //
  // Render data on the screen
  //

  // Clear the buffer and reset cursor
  display.clearDisplay();
  display.setTextColor(SSD1306_WHITE);
  display.setCursor(0,0);

  // Disaply temperature
  display.setTextSize(1);
  display.println(F("Temperature:"));
  display.setTextSize(2);
  display.print(temperature);
  display.print(" ");
  display.setTextSize(1);
  display.cp437(true);
  display.write(167);
  display.setTextSize(2);
  display.println(F("C"));

  display.setTextSize(1);
  display.println(F("====================="));

  // Disaply humidity
  display.println(F("Humidity:"));
  String humidityString = String(humidity) + String(F(" %"));
  display.setTextSize(2);
  display.println(humidityString.c_str());

  display.display();
  delay(SHOW_DATA_DELAY);
}