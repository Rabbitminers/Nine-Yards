/* tslint:disable */
/* eslint-disable */
/**
 * Nine Yards REST API
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: rabbitminers@zohomail.eu
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
import type { NotificationAction } from './NotificationAction';
import {
    NotificationActionFromJSON,
    NotificationActionFromJSONTyped,
    NotificationActionToJSON,
} from './NotificationAction';

/**
 * 
 * @export
 * @interface FullNotification
 */
export interface FullNotification {
    /**
     * 
     * @type {Array<NotificationAction>}
     * @memberof FullNotification
     */
    actions: Array<NotificationAction>;
    /**
     * The body (message) of the notification
     * @type {string}
     * @memberof FullNotification
     */
    body: string;
    /**
     * The datetime the notification was created and sent
     * @type {Date}
     * @memberof FullNotification
     */
    created: Date;
    /**
     * 
     * @type {string}
     * @memberof FullNotification
     */
    id: string;
    /**
     * Weather or not the recipient has read the notification
     * @type {boolean}
     * @memberof FullNotification
     */
    read: boolean;
    /**
     * 
     * @type {string}
     * @memberof FullNotification
     */
    userId: string;
}

/**
 * Check if a given object implements the FullNotification interface.
 */
export function instanceOfFullNotification(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "actions" in value;
    isInstance = isInstance && "body" in value;
    isInstance = isInstance && "created" in value;
    isInstance = isInstance && "id" in value;
    isInstance = isInstance && "read" in value;
    isInstance = isInstance && "userId" in value;

    return isInstance;
}

export function FullNotificationFromJSON(json: any): FullNotification {
    return FullNotificationFromJSONTyped(json, false);
}

export function FullNotificationFromJSONTyped(json: any, ignoreDiscriminator: boolean): FullNotification {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'actions': ((json['actions'] as Array<any>).map(NotificationActionFromJSON)),
        'body': json['body'],
        'created': (new Date(json['created'])),
        'id': json['id'],
        'read': json['read'],
        'userId': json['user_id'],
    };
}

export function FullNotificationToJSON(value?: FullNotification | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'actions': ((value.actions as Array<any>).map(NotificationActionToJSON)),
        'body': value.body,
        'created': (value.created.toISOString()),
        'id': value.id,
        'read': value.read,
        'user_id': value.userId,
    };
}

