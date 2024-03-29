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
/**
 * 
 * @export
 * @interface EditSubTask
 */
export interface EditSubTask {
    /**
     * The assigned member's user id
     * @type {string}
     * @memberof EditSubTask
     */
    assignee?: string | null;
    /**
     * The body (description of the sub task)
     * @type {string}
     * @memberof EditSubTask
     */
    body?: string | null;
    /**
     * Weather the sub task has been completed
     * @type {boolean}
     * @memberof EditSubTask
     */
    completed?: boolean | null;
    /**
     * The position of the sub task in task
     * @type {number}
     * @memberof EditSubTask
     */
    position?: number | null;
    /**
     * The influence of the sub task on total completion
     * @type {number}
     * @memberof EditSubTask
     */
    weight?: number | null;
}

/**
 * Check if a given object implements the EditSubTask interface.
 */
export function instanceOfEditSubTask(value: object): boolean {
    let isInstance = true;

    return isInstance;
}

export function EditSubTaskFromJSON(json: any): EditSubTask {
    return EditSubTaskFromJSONTyped(json, false);
}

export function EditSubTaskFromJSONTyped(json: any, ignoreDiscriminator: boolean): EditSubTask {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'assignee': !exists(json, 'assignee') ? undefined : json['assignee'],
        'body': !exists(json, 'body') ? undefined : json['body'],
        'completed': !exists(json, 'completed') ? undefined : json['completed'],
        'position': !exists(json, 'position') ? undefined : json['position'],
        'weight': !exists(json, 'weight') ? undefined : json['weight'],
    };
}

export function EditSubTaskToJSON(value?: EditSubTask | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'assignee': value.assignee,
        'body': value.body,
        'completed': value.completed,
        'position': value.position,
        'weight': value.weight,
    };
}

