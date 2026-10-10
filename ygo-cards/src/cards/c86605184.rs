//! Firewall Guardian
use super::prelude::*;

pub const ID: u32 = 86605184;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from GY
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_BE_MATERIAL);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Negate attack, and if you do, opponent's monster's ATK becomes 0
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    // return c:IsLocation(LOCATION_GRAVE) and r==REASON_LINK and c:GetReasonCard():IsRace(RACE_CYBERSE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && a.r == REASON_LINK && {
        let rc = card::get_reason_card(d, c).nil();
        card::is_race(d, rc, RACE_CYBERSE)
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
    {
        // Banish it if it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3300);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        card::register_effect(d, c, e1, true);
    }
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let df = duel::get_attack_target(d);
    // if not d or a:GetControler()==d:GetControler()
    //     or not d:IsFaceup() or not a:IsFaceup()
    //     or not a:IsType(TYPE_LINK) or not d:IsLinkMonster() then return false end
    let Some(df) = df else { return false };
    let at = at.nil();
    if card::get_controler(d, at) == card::get_controler(d, df)
        || !card::is_faceup(d, df)
        || !card::is_faceup(d, at)
        || !card::is_type(d, at, TYPE_LINK)
        || !card::is_link_monster(d, df)
    {
        return false;
    }
    // if a:IsControler(tp) then e:SetLabelObject(d) else e:SetLabelObject(a) end
    if card::is_controler(d, at, a.tp) {
        effect::set_label_object(d, a.e, Value::Card(df));
    } else {
        effect::set_label_object(d, a.e, Value::Card(at));
    }
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if tc and tc:IsRelateToBattle() and tc:IsFaceup() and Duel.NegateAttack() then
    if let Some(tc) = tc {
        if card::is_relate_to_battle(d, tc) && card::is_faceup(d, tc) && duel::negate_attack(d) {
            // ATK becomes 0
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
            effect::set_value(d, e1, 0);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
            // Unaffected by other card effects
            // local e2=Effect.CreateEffect(e:GetHandler())
            let e2 = effect::create_effect(d, c);
            effect::set_description(d, e2, 3100);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_range(d, e2, LOCATION_MZONE);
            effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
            effect::set_value_fn(d, e2, lua_fn(efilter));
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e2, false);
        }
    }
}
/// `function(e,te) return te:GetOwner()~=e:GetOwner() end`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let (e, te) = (a.e(0), a.e(1));
    effect::get_owner(d, te).nil() != effect::get_owner(d, e).nil()
}
