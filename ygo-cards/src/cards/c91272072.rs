//! Trickstar Band Sweet Guitar
//! scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 91272072;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRICKSTAR], material_setcode: &[SET_TRICKSTAR as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,s.matfilter,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_TRICKSTAR))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| matfilter(d, x, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_TRICKSTAR as u16], Some(fc), st, Some(tp)))),
        ],
    );
    // avoid damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(s.damval)
    effect::set_value_fn(d, e1, lua_fn(|d, a| damval(d, a)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // gain atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e2, EVENT_DAMAGE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // recycle
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_LEAVE_GRAVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,fc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_LINK,fc,sumtype,tp) and c:IsSetCard(SET_TRICKSTAR,fc,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_LINK, Some(fc), sumtype, Some(tp)) && card::is_set_card_ctx(d, c, &[SET_TRICKSTAR as u16], Some(fc), sumtype, Some(tp))
}
/// `s.damval(e,re,val,r,rp)`
fn damval(d: &mut Duel, a: Args) -> i64 {
    // local e = the effect (argument 0), re = argument 1 (may be nil), val = argument 2, r = argument 3
    let e = a.e(0);
    let val = a.int(2);
    let r = a.int(3) as u32;
    // if r&REASON_EFFECT==REASON_EFFECT and re and re:IsMonsterEffect() then
    if let Some(re) = a.v(1).as_effect() {
        if (r & REASON_EFFECT) == REASON_EFFECT && effect::is_monster_effect(d, re) {
            // local rc=re:GetHandler()
            let rc = effect::get_handler(d, re).nil();
            // if rc:IsFaceup() and rc:IsSetCard(SET_TRICKSTAR) and rc:IsLinkMonster()
            //     and rc:GetLinkedGroup():IsContains(e:GetHandler()) then
            let h = effect::get_handler(d, e).nil();
            if card::is_faceup(d, rc) && card::is_set_card(d, rc, &[SET_TRICKSTAR as u16]) && card::is_link_monster(d, rc) {
                let lg = card::get_linked_group(d, rc);
                if group::is_contains(&lg, h) {
                    // return val*2
                    return val * 2;
                }
            }
        }
    }
    // return val
    val
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and r&REASON_EFFECT==REASON_EFFECT and re:IsMonsterEffect() and re:GetHandler():IsSetCard(SET_TRICKSTAR)
    if a.ep as u8 == a.tp || (a.r & REASON_EFFECT) != REASON_EFFECT {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    card::is_set_card(d, rh, &[SET_TRICKSTAR as u16])
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetLabelObject(e)
        effect::set_label_object(d, e1, Value::Effect(a.e));
        // e1:SetValue(ev)
        effect::set_value(d, e1, a.ev as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_TRICKSTAR) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_TRICKSTAR as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetAttackedCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_attacked_count(d, h) > 0
}
/// Whether two effects' label objects are the same effect (`eff:GetLabelObject()==e:GetLabelObject()`)
fn same_label_object(d: &mut Duel, x: EffectId, y: EffectId) -> bool {
    effect::get_label_object(d, x).as_effect() == effect::get_label_object(d, y).as_effect()
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local effs={e:GetHandler():GetCardEffect(EFFECT_UPDATE_ATTACK)}
        let h = effect::get_handler(d, a.e).nil();
        let effs = card::get_card_effect(d, h, EFFECT_UPDATE_ATTACK);
        // for _,eff in ipairs(effs) do
        for eff in effs {
            // if eff:GetLabelObject()==e:GetLabelObject() then return true end
            if same_label_object(d, eff, a.e) {
                return true;
            }
        }
        // return false
        return false;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND as u64, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local chk=false
        let mut chk = false;
        // local effs={c:GetCardEffect(EFFECT_UPDATE_ATTACK)}
        let effs = card::get_card_effect(d, c, EFFECT_UPDATE_ATTACK);
        // for _,eff in ipairs(effs) do
        for eff in effs {
            // if eff:GetLabelObject()==e:GetLabelObject() then
            if same_label_object(d, eff, a.e) {
                // eff:Reset()
                effect::reset(d, eff);
                // chk=true
                chk = true;
            }
        }
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, x| thfilter(d, x))), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // if chk and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if chk && !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - a.tp, sg);
        }
    }
}
