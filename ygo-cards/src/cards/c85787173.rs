//! ジェネレーション・ネクスト
//! Generation Next
use super::prelude::*;

pub const ID: u32 = 85787173;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELEMENTAL_HERO, SET_NEO_SPACIAN, SET_KURIBOH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
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
/// `s.filter(c,e,tp,lc,lp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lc: i32, lp: i64) -> bool {
    // return (c:IsSetCard(SET_ELEMENTAL_HERO) or c:IsSetCard(SET_NEO_SPACIAN) or c:IsSetCard(SET_KURIBOH))
    //     and c:IsMonster() and c:IsAttackBelow(lp)
    //     and (c:IsAbleToHand() or (lc>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    (card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16, SET_NEO_SPACIAN as u16, SET_KURIBOH as u16]))
        && card::is_type(d, c, TYPE_MONSTER)
        && card::is_attack_below(d, c, lp as i32)
        && (card::is_able_to_hand(d, c, None)
            || (lc > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local lp=Duel.GetLP(1-tp)-Duel.GetLP(tp)
        let lp = duel::get_lp(d, 1 - tp) as i64 - duel::get_lp(d, tp) as i64;
        // return lp>0 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE),lp)
        if lp <= 0 {
            return false;
        }
        let lc = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| filter(d, x, a.e, tp, lc, lp)),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local lp=Duel.GetLP(1-tp)-Duel.GetLP(tp)
    let lp = duel::get_lp(d, 1 - tp) as i64 - duel::get_lp(d, tp) as i64;
    // if lp<=0 then return end
    if lp <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELF as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE),lp):GetFirst()
    let lc = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let necro = aux::necro_valley_filter(|d, x| filter(d, x, e, tp, lc, lp));
    let sg = duel::select_matching_card(d, tp, Some(&necro), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).unwrap_or_default();
    let tc = group::get_first(d, &sg);
    // if tc then
    if let Some(tc) = tc {
        // aux.ToHandOrElse(tc,tp,
        //     function() return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and tc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
        //     function() Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end,
        //     aux.Stringid(id,0))
        aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&|d, _x| {
                duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                    && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None)
            }),
            Some(&|d, _g| {
                duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                0
            }),
            Some(aux::stringid(ID, 0)),
        );
        // Cannot activate cards or the effects of cards with the same name
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(s.aclimit)
        effect::set_value_fn(d, e1, lua_fn(aclimit));
        // e1:SetLabel(tc:GetCode())
        let code = card::get_code(d, tc)[0];
        effect::set_label(d, e1, &[code as i64]);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local code=e:GetLabel()
    let code = effect::get_label(d, a.e(0));
    // return re:GetHandler():IsCode(code)
    let re = a.v(1).as_effect().nil();
    let rh = effect::get_handler(d, re).nil();
    card::is_code(d, rh, &[code as u32])
}
