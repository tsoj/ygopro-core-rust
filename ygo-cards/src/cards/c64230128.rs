//! Zaralaam the Dark Palace
//! 暗黒神殿ザララーム
use super::prelude::*;

pub const ID: u32 = 64230128;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ADVENTURER, 65952776, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // Prevent the opponent's cards and effects from being activated in the Battle Phase()
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(condition(actcon)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // Inflict damage to the opponent
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.bcon)
    effect::set_condition(d, e3, Some(condition(bcon)));
    // e3:SetTarget(s.btg)
    effect::set_target(d, e3, Some(cost_target(btg)));
    // e3:SetOperation(s.bop)
    effect::set_operation(d, e3, Some(operation(bop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);

    // Search a Field Spell
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.thcon)
    effect::set_condition(d, e4, Some(condition(thcon)));
    // e4:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e4:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetEquipGroup():IsExists(Card.IsCode,1,nil,65952776)
    let eq = card::get_equip_group(d, c);
    group::is_exists(d, &eq, &|d, x| card::is_code(d, x, &[65952776]), 1, NO_EX)
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase() and Duel.IsExistingMatchingCard(s.eqfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    if !duel::is_battle_phase(d, None) {
        return false;
    }
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&eqfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.bcon(e,tp,eg,ep,ev,re,r,rp)`
fn bcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let rc = group::get_first(d, &eg).nil();
    // return rc:IsRelateToBattle() and rc:IsStatus(STATUS_OPPO_BATTLE)
    //     and rc:IsFaceup() and rc:IsControler(tp) and rc:IsCode(TOKEN_ADVENTURER)
    card::is_relate_to_battle(d, rc) && card::is_status(d, rc, STATUS_OPPO_BATTLE) && card::is_faceup(d, rc) && card::is_controler(d, rc, a.tp) && card::is_code(d, rc, &[TOKEN_ADVENTURER])
}
/// `s.btg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn btg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local atk=eg:GetFirst():GetBattleTarget():GetTextAttack()
    let eg = a.eg.clone().nil();
    let rc = group::get_first(d, &eg).nil();
    let bt = card::get_battle_target(d, rc).nil();
    let atk = card::get_text_attack(d, bt);
    // if chk==0 then return atk>0 end
    if a.chk == Some(0) {
        return atk > 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,atk,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, atk as u8, 1 - tp, 0);
    true
}
/// `s.bop(e,tp,eg,ep,ev,re,r,rp)`
fn bop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(1-tp,eg:GetFirst():GetBattleTarget():GetTextAttack(),REASON_EFFECT)
    let eg = a.eg.clone().nil();
    let rc = group::get_first(d, &eg).nil();
    let bt = card::get_battle_target(d, rc).nil();
    let amount = card::get_text_attack(d, bt);
    duel::damage(d, 1 - a.tp, amount as i64, REASON_EFFECT, false, None);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(tp,id)>0
    duel::get_flag_effect(d, a.tp, ID) > 0
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:ListsCode(TOKEN_ADVENTURER) and c:IsFieldSpell() and c:IsAbleToHand() and not c:IsCode(id)
    card::lists_code(d, c, &[TOKEN_ADVENTURER]) && card::is_field_spell(d, c) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(thfilter)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
