/**
 * @file sensor22_sensor_read_temp.ino
 * @brief Based on the example from Adafruit DHT sensor Arduino library:
 *      https://github.com/adafruit/DHT-sensor-library/blob/master/examples/DHTtester/DHTtester.ino
 * @details The example reads temperature and humidity from the DHT22 sensor and dumps the result to serial
 *      REQUIRES the following Arduino libraries:
 *        - DHT Sensor Library: https://github.com/adafruit/DHT-sensor-library
 *        - Adafruit Unified Sensor Lib: https://github.com/adafruit/Adafruit_Sensor
 */

#include "DHT.h"

// Setup DHT22 sensor
constexpr const uint8_t DHT_SENSOR_TYPE = DHT22; // DHT22 (AM2302)
constexpr const uint8_t DHT_DATA_PIN = 2;
DHT sensor(DHT_DATA_PIN, DHT_SENSOR_TYPE);

#if defined(ARDUINO_ARCH_AVR)
    #define debug  Serial
#elif defined(ARDUINO_ARCH_SAMD) ||  defined(ARDUINO_ARCH_SAM)
    #define debug  SerialUSB
#else
    #define debug  Serial
#endif

constexpr const unsigned long SERIAL_BAUD_RATE = 115200;
constexpr const unsigned long MEASUREMENT_DELAY_MS = 2000;

void setup() {
    debug.begin(SERIAL_BAUD_RATE);
    sensor.begin();
}

void loop() {
    const float temperature = sensor.readTemperature();
    const float humidity = sensor.readHumidity();

    if (isnan(temperature) || isnan(humidity)) {
        Serial.println(F("Failed to read from DHT sensor!"));
        return;
    }

    debug.print(F("Humidity: "));
    debug.print(humidity);
    debug.print(F(" %\t"));
    debug.print(F("Temperature: "));
    debug.print(temperature);
    debug.println(F(" *C"));

    delay(MEASUREMENT_DELAY_MS);
}