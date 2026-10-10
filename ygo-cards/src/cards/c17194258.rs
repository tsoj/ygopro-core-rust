//! Fusion Conscription
use super::prelude::*;

pub const ID: u32 = 17194258;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c,tp)`
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c.material and c:IsType(TYPE_FUSION) and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,c)
    !crate::class::material(d, c).is_empty()
        && card::is_type(d, c, TYPE_FUSION)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, c)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.filter2(c,fc)`
fn filter2(d: &mut Duel, c: CardId, fc: CardId) -> bool {
    // if c:IsForbidden() or not c:IsAbleToHand() then return false end
    if card::is_forbidden(d, c) || !card::is_able_to_hand(d, c, None) {
        return false;
    }
    // return c:IsCode(table.unpack(fc.material))
    let material = crate::class::material(d, fc);
    card::is_code(d, c, &material)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_EXTRA,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter1(d, c, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_EXTRA,0,1,1,nil,tp)
    let cg = duel::select_matching_card(d, a.tp, Some(&|d, c| filter1(d, c, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #cg==0 then return end
    if cg.is_empty() {
        return;
    }
    // Duel.ConfirmCards(1-tp,cg)
    duel::confirm_cards(d, 1 - a.tp, &cg);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter2),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,cg:GetFirst())
    let fc = group::get_first(d, &cg).nil();
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| filter2(d, c, fc))),
        a.tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // local tc=g:GetFirst()
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    // if Duel.SendtoHand(tc,nil,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_HAND) then
    if duel::send_to_hand(d, tc, None, REASON_EFFECT, None) != 0 && card::is_location(d, tc, LOCATION_HAND) {
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, tc);
        // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
        if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
            return;
        }
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(s.sumlimit)
        effect::set_target(d, e1, Some(effect_card_fn(sumlimit)));
        // e1:SetLabel(tc:GetCode())
        let code = card::get_code(d, tc)[0];
        effect::set_label(d, e1, &[code as i64]);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetCode(EFFECT_CANNOT_MSET)
        effect::set_code(d, e3, EFFECT_CANNOT_MSET);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, a.tp);
        // local e4=e1:Clone()
        let e4 = effect::clone(d, e1);
        // e4:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
        // e4:SetValue(s.aclimit)
        effect::set_value_fn(d, e4, lua_fn(aclimit));
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, a.tp);
    }
}
/// `s.sumlimit(e,c)`
fn sumlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsCode(e:GetLabel())
    let code = effect::get_label(d, e) as u32;
    card::is_code(d, c, &[code])
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    // return re:GetHandler():IsCode(e:GetLabel()) and re:IsMonsterEffect()
    let h = effect::get_handler(d, re).nil();
    let code = effect::get_label(d, e) as u32;
    card::is_code(d, h, &[code]) && effect::is_monster_effect(d, re)
}
