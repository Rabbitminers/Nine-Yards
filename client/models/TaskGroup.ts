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
 * @interface TaskGroup
 */
export interface TaskGroup {
    /**
     * 
     * @type {string}
     * @memberof TaskGroup
     */
    id: string;
    /**
     * The task group's name (3 -> 30 characters)
     * @type {string}
     * @memberof TaskGroup
     */
    name: string;
    /**
     * The position of the task group in the project
     * starting from zero
     * @type {number}
     * @memberof TaskGroup
     */
    position: number;
    /**
     * 
     * @type {string}
     * @memberof TaskGroup
     */
    projectId: string;
}

/**
 * Check if a given object implements the TaskGroup interface.
 */
export function instanceOfTaskGroup(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "id" in value;
    isInstance = isInstance && "name" in value;
    isInstance = isInstance && "position" in value;
    isInstance = isInstance && "projectId" in value;

    return isInstance;
}

export function TaskGroupFromJSON(json: any): TaskGroup {
    return TaskGroupFromJSONTyped(json, false);
}

export function TaskGroupFromJSONTyped(json: any, ignoreDiscriminator: boolean): TaskGroup {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'id': json['id'],
        'name': json['name'],
        'position': json['position'],
        'projectId': json['project_id'],
    };
}

export function TaskGroupToJSON(value?: TaskGroup | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'id': value.id,
        'name': value.name,
        'position': value.position,
        'project_id': value.projectId,
    };
}

