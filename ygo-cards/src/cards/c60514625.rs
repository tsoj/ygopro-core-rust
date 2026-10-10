//! Ecole de Zone
use super::prelude::*;

pub const ID: u32 = 60514625;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[60514626], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Token
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.tkcon)
    effect::set_condition(d, e1, Some(condition(tkcon)));
    // e1:SetTarget(s.tktg)
    effect::set_target(d, e1, Some(cost_target(tktg)));
    // e1:SetOperation(s.tkop)
    effect::set_operation(d, e1, Some(operation(tkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy "Mask Token"
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetCondition(s.descon)
    effect::set_condition(d, e4, Some(condition(descon)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// s.tkcon(e,tp,eg,ep,ev,re,r,rp)
fn tkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if #eg~=1 then return false end
    let eg = a.eg.clone().nil();
    if group::get_count(&eg) != 1 {
        return false;
    }
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &eg).nil();
    // return tc:IsFaceup() and tc:IsSummonPlayer(Duel.GetTurnPlayer())
    card::is_faceup(d, tc) && card::is_summon_player(d, tc, duel::get_turn_player(d))
}
// s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    true
}
// s.tkop(e,tp,eg,ep,ev,re,r,rp)
fn tkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    // if tc:IsFaceup() and tc:IsLocation(LOCATION_MZONE) and not tc:IsImmuneToEffect(e) then
    if card::is_faceup(d, tc) && card::is_location(d, tc, LOCATION_MZONE) && !card::is_immune_to_effect(d, tc, a.e) {
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc);
        // local def=tc:GetDefense()
        let def = card::get_defense(d, tc);
        // local p=tc:GetControler()
        let p = card::get_controler(d, tc);
        // if Duel.Destroy(tc,REASON_EFFECT)==0 then return end
        if duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 {
            return;
        }
        // if Duel.GetLocationCount(p,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(p,id+1,0,TYPES_TOKEN,atk,def,1,RACE_SPELLCASTER,ATTRIBUTE_DARK) then
        if duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(
                d,
                p,
                ID + 1,
                duel::MonsterData {
                    setcodes: Some(vec![0]),
                    type_: Some(TYPES_TOKEN),
                    attack: Some(atk),
                    defense: Some(def),
                    level: Some(1),
                    race: Some(RACE_SPELLCASTER),
                    attribute: Some(ATTRIBUTE_DARK),
                },
                None,
                None,
                0,
            )
        {
            // local token=Duel.CreateToken(p,id+1)
            let token = duel::create_token(d, p, ID + 1).nil();
            // if Duel.SpecialSummonStep(token,0,p,p,false,false,POS_FACEUP) then
            if duel::special_summon_step(d, token, 0, p, p, false, false, POS_FACEUP as u8, None) {
                // Set ATK/DEF
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_SET_ATTACK)
                effect::set_code(d, e1, EFFECT_SET_ATTACK);
                // e1:SetValue(atk)
                effect::set_value(d, e1, atk as i64);
                // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
                effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
                // token:RegisterEffect(e1)
                card::register_effect(d, token, e1, false);
                // local e2=e1:Clone()
                let e2 = effect::clone(d, e1);
                // e2:SetCode(EFFECT_SET_DEFENSE)
                effect::set_code(d, e2, EFFECT_SET_DEFENSE);
                // e2:SetValue(def)
                effect::set_value(d, e2, def as i64);
                // token:RegisterEffect(e2)
                card::register_effect(d, token, e2, false);
                // Cannot attack
                // local e3=Effect.CreateEffect(c)
                let e3 = effect::create_effect(d, c);
                // e3:SetDescription(3207)
                effect::set_description(d, e3, 3207);
                // e3:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                // e3:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
                effect::set_code(d, e3, EFFECT_CANNOT_DIRECT_ATTACK);
                // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                // e3:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
                effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
                // token:RegisterEffect(e3)
                card::register_effect(d, token, e3, false);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
// s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousLocation(LOCATION_ONFIELD)
    card::is_previous_position(d, c, POS_FACEUP) && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
// s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,id+1),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,0,nil)
    let g = duel::get_matching_group(
        d,
        Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[ID + 1])),
        a.tp,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        NO_EX,
    );
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
