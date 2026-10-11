//! Altergeist Kidolga
//! --オルターガイスト・キードゥルガー
use super::prelude::*;

pub const ID: u32 = 76685519;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ALTERGEIST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ALTERGEIST),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_ALTERGEIST as u16], sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // Special Summon 1 monster from your opponent's GY to your zone this card points to
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Altergeist" card from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let c = effect::get_handler(d, a.e).nil();
    // return ep==1-tp and tc:IsControler(tp) and tc:IsSetCard(SET_ALTERGEIST) and tc~=e:GetHandler()
    if a.ep != 1 - a.tp {
        return false;
    }
    let tc = group::get_first(d, &eg).nil();
    card::is_controler(d, tc, a.tp) && card::is_set_card(d, tc, &[SET_ALTERGEIST as u16]) && tc != c
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local zone=e:GetHandler():GetLinkedZone(tp)&ZONES_MMZ
    let c = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, c, Some(tp)) & ZONES_MMZ;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(1-tp) and s.spfilter(chkc,e,tp,zone) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - tp) && spfilter(d, chkc, a.e, tp, zone);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter,tp,0,LOCATION_GRAVE,1,nil,e,tp,zone) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, tp, zone)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,0,LOCATION_GRAVE,1,1,nil,e,tp,zone)
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, zone)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=c:GetLinkedZone(tp)&ZONES_MMZ
    let zone = card::get_linked_zone(d, c, Some(tp)) & ZONES_MMZ;
    // if not c:IsRelateToEffect(e) or zone==0 then return end
    if !card::is_relate_to_effect(d, c, e) || zone == 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP,zone)>0 then
    if card::is_relate_to_effect(d, tc, e)
        && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, Some(zone)) > 0
    {
        // if c:GetAttackAnnouncedCount()>0 then
        if card::get_attack_announced_count(d, c) > 0 {
            // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
            card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        }
        // tc:CreateRelation(c,RESET_EVENT|RESETS_STANDARD)
        card::create_relation(d, tc, c, RESET_EVENT | RESETS_STANDARD);
        // The Special Summoned monster cannot attack
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetCondition(function(e) return not e:GetHandler():HasFlagEffect(id) end)
        effect::set_condition(d, e1, Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            !card::has_flag_effect(d, h, ID, None)
        })));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // Track this card's attacks
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
        // e2:SetLabelObject(tc)
        effect::set_label_object(d, e2, Value::Card(tc));
        // e2:SetCondition(function(e) return e:GetLabelObject():IsRelateToCard(e:GetHandler()) end)
        effect::set_condition(d, e2, Some(condition(|d, a| {
            let lo = effect::get_label_object(d, a.e).as_card().nil();
            let h = effect::get_handler(d, a.e).nil();
            card::is_relate_to_card(d, lo, h)
        })));
        // e2:SetOperation(function(e) e:GetLabelObject():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1) end)
        effect::set_operation(d, e2, Some(operation(|d, a| {
            let lo = effect::get_label_object(d, a.e).as_card().nil();
            card::register_flag_effect(d, lo, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        })));
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ALTERGEIST) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
