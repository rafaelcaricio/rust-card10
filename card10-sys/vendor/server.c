#include "modules/log.h"
#include "epicardium.h"

void __api_dispatch_call(uint32_t id, void*epc__apistub_buffer)
{
        switch (id) {
        case API_INTERRUPT_ENABLE:
                *((int*)epc__apistub_buffer) = epic_interrupt_enable(
                        *(api_int_id_t*)(epc__apistub_buffer + 0)
                );
                break;
        case API_INTERRUPT_DISABLE:
                *((int*)epc__apistub_buffer) = epic_interrupt_disable(
                        *(api_int_id_t*)(epc__apistub_buffer + 0)
                );
                break;
        case API_SYSTEM_EXIT:
                __epic_exit(
                        *(int*)(epc__apistub_buffer + 0)
                );
                break;
        case API_SYSTEM_EXEC:
                *((int*)epc__apistub_buffer) = __epic_exec(
                        *(char **)(epc__apistub_buffer + 0)
                );
                break;
        case API_SYSTEM_RESET:
                epic_system_reset(
                );
                break;
        case API_BATTERY_VOLTAGE:
                *((int*)epc__apistub_buffer) = epic_read_battery_voltage(
                        *(float **)(epc__apistub_buffer + 0)
                );
                break;
        case API_BATTERY_CURRENT:
                *((int*)epc__apistub_buffer) = epic_read_battery_current(
                        *(float **)(epc__apistub_buffer + 0)
                );
                break;
        case API_CHARGEIN_VOLTAGE:
                *((int*)epc__apistub_buffer) = epic_read_chargein_voltage(
                        *(float **)(epc__apistub_buffer + 0)
                );
                break;
        case API_CHARGEIN_CURRENT:
                *((int*)epc__apistub_buffer) = epic_read_chargein_current(
                        *(float **)(epc__apistub_buffer + 0)
                );
                break;
        case API_SYSTEM_VOLTAGE:
                *((int*)epc__apistub_buffer) = epic_read_system_voltage(
                        *(float **)(epc__apistub_buffer + 0)
                );
                break;
        case API_THERMISTOR_VOLTAGE:
                *((int*)epc__apistub_buffer) = epic_read_thermistor_voltage(
                        *(float **)(epc__apistub_buffer + 0)
                );
                break;
        case API_UART_WRITE_STR:
                epic_uart_write_str(
                        *(const char **)(epc__apistub_buffer + 0),
                        *(intptr_t*)(epc__apistub_buffer + sizeof(const char *))
                );
                break;
        case API_UART_READ_CHAR:
                *((int*)epc__apistub_buffer) = epic_uart_read_char(
                );
                break;
        case API_UART_READ_STR:
                *((int*)epc__apistub_buffer) = epic_uart_read_str(
                        *(char **)(epc__apistub_buffer + 0),
                        *(size_t*)(epc__apistub_buffer + sizeof(char *))
                );
                break;
        case API_BUTTONS_READ:
                *((uint8_t*)epc__apistub_buffer) = epic_buttons_read(
                        *(uint8_t*)(epc__apistub_buffer + 0)
                );
                break;
        case API_GPIO_SET_PIN_MODE:
                *((int*)epc__apistub_buffer) = epic_gpio_set_pin_mode(
                        *(uint8_t*)(epc__apistub_buffer + 0),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(uint8_t))
                );
                break;
        case API_GPIO_GET_PIN_MODE:
                *((int*)epc__apistub_buffer) = epic_gpio_get_pin_mode(
                        *(uint8_t*)(epc__apistub_buffer + 0)
                );
                break;
        case API_GPIO_WRITE_PIN:
                *((int*)epc__apistub_buffer) = epic_gpio_write_pin(
                        *(uint8_t*)(epc__apistub_buffer + 0),
                        *(_Bool*)(epc__apistub_buffer + sizeof(uint8_t))
                );
                break;
        case API_GPIO_READ_PIN:
                *((int*)epc__apistub_buffer) = epic_gpio_read_pin(
                        *(uint8_t*)(epc__apistub_buffer + 0)
                );
                break;
        case API_LEDS_SET:
                epic_leds_set(
                        *(int*)(epc__apistub_buffer + 0),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(int)),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(int) + sizeof(uint8_t)),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(int) + sizeof(uint8_t) + sizeof(uint8_t))
                );
                break;
        case API_LEDS_SET_HSV:
                epic_leds_set_hsv(
                        *(int*)(epc__apistub_buffer + 0),
                        *(float*)(epc__apistub_buffer + sizeof(int)),
                        *(float*)(epc__apistub_buffer + sizeof(int) + sizeof(float)),
                        *(float*)(epc__apistub_buffer + sizeof(int) + sizeof(float) + sizeof(float))
                );
                break;
        case API_LEDS_SET_ALL:
                epic_leds_set_all(
                        *(uint8_t **)(epc__apistub_buffer + 0),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(uint8_t *))
                );
                break;
        case API_LEDS_SET_ALL_HSV:
                epic_leds_set_all_hsv(
                        *(float **)(epc__apistub_buffer + 0),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(float *))
                );
                break;
        case API_LEDS_PREP:
                epic_leds_prep(
                        *(int*)(epc__apistub_buffer + 0),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(int)),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(int) + sizeof(uint8_t)),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(int) + sizeof(uint8_t) + sizeof(uint8_t))
                );
                break;
        case API_LEDS_PREP_HSV:
                epic_leds_prep_hsv(
                        *(int*)(epc__apistub_buffer + 0),
                        *(float*)(epc__apistub_buffer + sizeof(int)),
                        *(float*)(epc__apistub_buffer + sizeof(int) + sizeof(float)),
                        *(float*)(epc__apistub_buffer + sizeof(int) + sizeof(float) + sizeof(float))
                );
                break;
        case API_LEDS_DIM_BOTTOM:
                epic_leds_dim_bottom(
                        *(uint8_t*)(epc__apistub_buffer + 0)
                );
                break;
        case API_LEDS_DIM_TOP:
                epic_leds_dim_top(
                        *(uint8_t*)(epc__apistub_buffer + 0)
                );
                break;
        case API_LEDS_SET_POWERSAVE:
                epic_leds_set_powersave(
                        *(_Bool*)(epc__apistub_buffer + 0)
                );
                break;
        case API_LEDS_UPDATE:
                epic_leds_update(
                );
                break;
        case API_LEDS_SET_ROCKET:
                epic_leds_set_rocket(
                        *(int*)(epc__apistub_buffer + 0),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(int))
                );
                break;
        case API_LEDS_SET_FLASHLIGHT:
                epic_set_flashlight(
                        *(_Bool*)(epc__apistub_buffer + 0)
                );
                break;
        case API_LEDS_SET_GAMMA_TABLE:
                epic_leds_set_gamma_table(
                        *(uint8_t*)(epc__apistub_buffer + 0),
                        *(uint8_t **)(epc__apistub_buffer + sizeof(uint8_t))
                );
                break;
        case API_LEDS_CLEAR_ALL:
                epic_leds_clear_all(
                        *(uint8_t*)(epc__apistub_buffer + 0),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(uint8_t)),
                        *(uint8_t*)(epc__apistub_buffer + sizeof(uint8_t) + sizeof(uint8_t))
                );
                break;
        case API_BME680_INIT:
                *((int*)epc__apistub_buffer) = epic_bme680_init(
                );
                break;
        case API_BME680_DEINIT:
                *((int*)epc__apistub_buffer) = epic_bme680_deinit(
                );
                break;
        case API_BME680_GET_DATA:
                *((int*)epc__apistub_buffer) = epic_bme680_read_sensors(
                        *(struct bme680_sensor_data **)(epc__apistub_buffer + 0)
                );
                break;
        case API_PERSONAL_STATE_SET:
                *((int*)epc__apistub_buffer) = epic_personal_state_set(
                        *(uint8_t*)(epc__apistub_buffer + 0),
                        *(_Bool*)(epc__apistub_buffer + sizeof(uint8_t))
                );
                break;
        case API_PERSONAL_STATE_GET:
                *((int*)epc__apistub_buffer) = epic_personal_state_get(
                );
                break;
        case API_PERSONAL_STATE_IS_PERSISTENT:
                *((int*)epc__apistub_buffer) = epic_personal_state_is_persistent(
                );
                break;
        case API_STREAM_READ:
                *((int*)epc__apistub_buffer) = epic_stream_read(
                        *(int*)(epc__apistub_buffer + 0),
                        *(void **)(epc__apistub_buffer + sizeof(int)),
                        *(size_t*)(epc__apistub_buffer + sizeof(int) + sizeof(void *))
                );
                break;
        case API_BHI160_ENABLE:
                *((int*)epc__apistub_buffer) = epic_bhi160_enable_sensor(
                        *(enum bhi160_sensor_type*)(epc__apistub_buffer + 0),
                        *(struct bhi160_sensor_config **)(epc__apistub_buffer + sizeof(enum bhi160_sensor_type))
                );
                break;
        case API_BHI160_DISABLE:
                *((int*)epc__apistub_buffer) = epic_bhi160_disable_sensor(
                        *(enum bhi160_sensor_type*)(epc__apistub_buffer + 0)
                );
                break;
        case API_BHI160_DISABLE_ALL:
                epic_bhi160_disable_all_sensors(
                );
                break;
        case API_VIBRA_SET:
                epic_vibra_set(
                        *(int*)(epc__apistub_buffer + 0)
                );
                break;
        case API_VIBRA_VIBRATE:
                epic_vibra_vibrate(
                        *(int*)(epc__apistub_buffer + 0)
                );
                break;
        case API_DISP_OPEN:
                *((int*)epc__apistub_buffer) = epic_disp_open(
                );
                break;
        case API_DISP_CLOSE:
                *((int*)epc__apistub_buffer) = epic_disp_close(
                );
                break;
        case API_DISP_UPDATE:
                *((int*)epc__apistub_buffer) = epic_disp_update(
                );
                break;
        case API_DISP_PRINT:
                *((int*)epc__apistub_buffer) = epic_disp_print(
                        *(uint16_t*)(epc__apistub_buffer + 0),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t)),
                        *(const char **)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(const char *)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(const char *) + sizeof(uint16_t))
                );
                break;
        case API_DISP_CLEAR:
                *((int*)epc__apistub_buffer) = epic_disp_clear(
                        *(uint16_t*)(epc__apistub_buffer + 0)
                );
                break;
        case API_DISP_PIXEL:
                *((int*)epc__apistub_buffer) = epic_disp_pixel(
                        *(uint16_t*)(epc__apistub_buffer + 0),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t))
                );
                break;
        case API_DISP_LINE:
                *((int*)epc__apistub_buffer) = epic_disp_line(
                        *(uint16_t*)(epc__apistub_buffer + 0),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(enum disp_linestyle*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(enum disp_linestyle))
                );
                break;
        case API_DISP_RECT:
                *((int*)epc__apistub_buffer) = epic_disp_rect(
                        *(uint16_t*)(epc__apistub_buffer + 0),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(enum disp_fillstyle*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(enum disp_fillstyle))
                );
                break;
        case API_DISP_CIRC:
                *((int*)epc__apistub_buffer) = epic_disp_circ(
                        *(uint16_t*)(epc__apistub_buffer + 0),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(enum disp_fillstyle*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)),
                        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(enum disp_fillstyle))
                );
                break;
        case API_DISP_FRAMEBUFFER:
                *((int*)epc__apistub_buffer) = epic_disp_framebuffer(
                        *(union disp_framebuffer **)(epc__apistub_buffer + 0)
                );
                break;
        case API_DISP_BACKLIGHT:
                *((int*)epc__apistub_buffer) = epic_disp_backlight(
                        *(uint16_t*)(epc__apistub_buffer + 0)
                );
                break;
        case API_LIGHT_SENSOR_RUN:
                *((int*)epc__apistub_buffer) = epic_light_sensor_run(
                );
                break;
        case API_LIGHT_SENSOR_GET:
                *((int*)epc__apistub_buffer) = epic_light_sensor_get(
                        *(uint16_t**)(epc__apistub_buffer + 0)
                );
                break;
        case API_LIGHT_SENSOR_STOP:
                *((int*)epc__apistub_buffer) = epic_light_sensor_stop(
                );
                break;
        case API_LIGHT_SENSOR_READ:
                *((uint16_t*)epc__apistub_buffer) = epic_light_sensor_read(
                );
                break;
        case API_FILE_OPEN:
                *((int*)epc__apistub_buffer) = epic_file_open(
                        *(const char**)(epc__apistub_buffer + 0),
                        *(const char**)(epc__apistub_buffer + sizeof(const char*))
                );
                break;
        case API_FILE_CLOSE:
                *((int*)epc__apistub_buffer) = epic_file_close(
                        *(int*)(epc__apistub_buffer + 0)
                );
                break;
        case API_FILE_READ:
                *((int*)epc__apistub_buffer) = epic_file_read(
                        *(int*)(epc__apistub_buffer + 0),
                        *(void**)(epc__apistub_buffer + sizeof(int)),
                        *(size_t*)(epc__apistub_buffer + sizeof(int) + sizeof(void*))
                );
                break;
        case API_FILE_WRITE:
                *((int*)epc__apistub_buffer) = epic_file_write(
                        *(int*)(epc__apistub_buffer + 0),
                        *(const void**)(epc__apistub_buffer + sizeof(int)),
                        *(size_t*)(epc__apistub_buffer + sizeof(int) + sizeof(const void*))
                );
                break;
        case API_FILE_FLUSH:
                *((int*)epc__apistub_buffer) = epic_file_flush(
                        *(int*)(epc__apistub_buffer + 0)
                );
                break;
        case API_FILE_SEEK:
                *((int*)epc__apistub_buffer) = epic_file_seek(
                        *(int*)(epc__apistub_buffer + 0),
                        *(long*)(epc__apistub_buffer + sizeof(int)),
                        *(int*)(epc__apistub_buffer + sizeof(int) + sizeof(long))
                );
                break;
        case API_FILE_TELL:
                *((int*)epc__apistub_buffer) = epic_file_tell(
                        *(int*)(epc__apistub_buffer + 0)
                );
                break;
        case API_FILE_STAT:
                *((int*)epc__apistub_buffer) = epic_file_stat(
                        *(const char**)(epc__apistub_buffer + 0),
                        *(struct epic_stat**)(epc__apistub_buffer + sizeof(const char*))
                );
                break;
        case API_FILE_OPENDIR:
                *((int*)epc__apistub_buffer) = epic_file_opendir(
                        *(const char**)(epc__apistub_buffer + 0)
                );
                break;
        case API_FILE_READDIR:
                *((int*)epc__apistub_buffer) = epic_file_readdir(
                        *(int*)(epc__apistub_buffer + 0),
                        *(struct epic_stat**)(epc__apistub_buffer + sizeof(int))
                );
                break;
        case API_FILE_UNLINK:
                *((int*)epc__apistub_buffer) = epic_file_unlink(
                        *(const char**)(epc__apistub_buffer + 0)
                );
                break;
        case API_FILE_RENAME:
                *((int*)epc__apistub_buffer) = epic_file_rename(
                        *(const char **)(epc__apistub_buffer + 0),
                        *(const char**)(epc__apistub_buffer + sizeof(const char *))
                );
                break;
        case API_FILE_MKDIR:
                *((int*)epc__apistub_buffer) = epic_file_mkdir(
                        *(const char **)(epc__apistub_buffer + 0)
                );
                break;
        case API_RTC_GET_SECONDS:
                *((uint32_t*)epc__apistub_buffer) = epic_rtc_get_seconds(
                );
                break;
        case API_RTC_GET_MILLISECONDS:
                *((uint64_t*)epc__apistub_buffer) = epic_rtc_get_milliseconds(
                );
                break;
        case API_RTC_SET_MILLISECONDS:
                epic_rtc_set_milliseconds(
                        *(uint64_t*)(epc__apistub_buffer + 0)
                );
                break;
        case API_RTC_SCHEDULE_ALARM:
                *((int*)epc__apistub_buffer) = epic_rtc_schedule_alarm(
                        *(uint32_t*)(epc__apistub_buffer + 0)
                );
                break;
        case API_TRNG_READ:
                *((int*)epc__apistub_buffer) = epic_trng_read(
                        *(uint8_t **)(epc__apistub_buffer + 0),
                        *(size_t*)(epc__apistub_buffer + sizeof(uint8_t *))
                );
                break;
        case API_MAX30001_ENABLE:
                *((int*)epc__apistub_buffer) = epic_max30001_enable_sensor(
                        *(struct max30001_sensor_config **)(epc__apistub_buffer + 0)
                );
                break;
        case API_MAX30001_DISABLE:
                *((int*)epc__apistub_buffer) = epic_max30001_disable_sensor(
                );
                break;
        case API_USB_SHUTDOWN:
                *((int*)epc__apistub_buffer) = epic_usb_shutdown(
                );
                break;
        case API_USB_STORAGE:
                *((int*)epc__apistub_buffer) = epic_usb_storage(
                );
                break;
        case API_USB_CDCACM:
                *((int*)epc__apistub_buffer) = epic_usb_cdcacm(
                );
                break;
        default:
                /* TODO: Better error handling */
                LOG_ERR("api-dispatcher", "API function 0x%lx is unknown!!", id);
                break;
        }
}
