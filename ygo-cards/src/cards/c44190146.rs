//! Grandpa Demetto
//! デメット爺さん
//! Logical Nonsense
use super::prelude::*;

pub const ID: u32 = 44190146;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_PRINCESS_COLOGNE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_DETACH_EVENT)
    duel::enable_global_flag(d, GLOBALFLAG_DETACH_EVENT);
    // Special Summon up to 2 Normal Monsters with 0 ATK or DEF from your GY, each as a Level 8 DARK monster in Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // Destroy 1 monster your opponent controls and inflict damage to your opponent equal to the Xyz Monster's Rank x 300
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local xyzg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,CARD_PRINCESS_COLOGNE),tp,LOCATION_MZONE,0,nil)
    let xyzg = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_PRINCESS_COLOGNE])),
        a.tp,
        LOCATION_MZONE,
        0,
        NO_EX,
    );
    // if chk==0 then return #xyzg>0 and Duel.CheckRemoveOverlayCard(tp,1,0,1,REASON_COST,xyzg) end
    if a.chk == Some(0) {
        return !xyzg.is_empty() && duel::check_remove_overlay_card(d, a.tp, 1, 0, 1, REASON_COST, Some(&xyzg));
    }
    // Duel.RemoveOverlayCard(tp,1,0,1,1,REASON_COST,xyzg)
    duel::remove_overlay_card(d, a.tp, 1, 0, 1, 1, REASON_COST, Some(&xyzg));
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:AssumeProperty(ASSUME_LEVEL,8)
    card::assume_property(d, c, ASSUME_LEVEL, 8);
    // c:AssumeProperty(ASSUME_ATTRIBUTE,ATTRIBUTE_DARK)
    card::assume_property(d, c, ASSUME_ATTRIBUTE, ATTRIBUTE_DARK as u64);
    // return c:IsType(TYPE_NORMAL) and (c:IsAttack(0) or c:IsDefense(0)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_type(d, c, TYPE_NORMAL)
        && (card::is_attack(d, c, &[0]) || card::is_defense(d, c, &[0]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ct = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ct<=0 then return end
    if ct <= 0 {
        return;
    }
    // ct=math.min(ct,2)
    ct = ct.min(2);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,ct,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))),
        a.tp,
        LOCATION_GRAVE,
        0,
        1,
        ct as u16,
        false,
        NO_EX,
    )
    .nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for sc in g:Iter() do
    for sc in group::iter(d, &g) {
        // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
        if duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
            // Special Summon each one as a Level 8 DARK monster
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
            // e1:SetValue(ATTRIBUTE_DARK)
            effect::set_value(d, e1, ATTRIBUTE_DARK as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e2, EFFECT_CHANGE_LEVEL);
            // e2:SetValue(8)
            effect::set_value(d, e2, 8);
            // sc:RegisterEffect(e2)
            card::register_effect(d, sc, e2, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=re:GetChainData().cost_detached_materials
    let re = a.re.nil();
    let dg = chain::effect_data(d, re, None, chain::field::COST_DETACHED_MATERIALS).as_group().cloned();
    // return dg and re:GetHandler():IsControler(tp) and re:IsActiveType(TYPE_XYZ)
    //     and dg:IsExists(Card.IsOriginalType,1,nil,TYPE_NORMAL)
    match dg {
        None => false,
        Some(dg) => {
            let h = effect::get_handler(d, re).nil();
            card::is_controler(d, h, a.tp)
                && effect::is_active_type(d, re, TYPE_XYZ)
                && group::is_exists(d, &dg, &|d, c| card::is_original_type(d, c, TYPE_NORMAL), 1, NO_EX)
        }
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // if chk==0 then return rc:IsCanBeEffectTarget(e) and Duel.IsExistingTarget(nil,tp,0,LOCATION_MZONE,1,rc) end
    if a.chk == Some(0) {
        return card::is_can_be_effect_target(d, rc, Some(a.e)) && duel::is_existing_target(d, None, a.tp, 0, LOCATION_MZONE, 1, rc.into());
    }
    // Duel.SetTargetCard(rc)
    duel::set_target_card(d, rc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,nil,tp,0,LOCATION_MZONE,1,1,rc):GetFirst()
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, rc.into()).nil();
    let tc = group::get_first(d, &g);
    // local cd=e:GetChainData()
    // cd.xyz_card=rc
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Card(rc));
    // cd.opp_card=tc
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 1, tc.map_or(Value::Nil, Value::Card));
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,rc:GetRank()*300)
    let rank = card::get_rank(d, rc) as i32;
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, rank * 300);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local cd=e:GetChainData()
    // if cd.opp_card:IsRelateToEffect(e)
    let opp = chain::effect_data(d, a.e, None, chain::field::CARD + 1).as_card().nil();
    if card::is_relate_to_effect(d, opp, a.e)
        // and cd.opp_card:IsControler(1-tp)
        && card::is_controler(d, opp, 1 - a.tp)
        // and Duel.Destroy(cd.opp_card,REASON_EFFECT)>0
        && duel::destroy(d, opp, REASON_EFFECT, None, None) > 0
    {
        // and cd.xyz_card:IsRelateToEffect(e)
        let xyz = chain::effect_data(d, a.e, None, chain::field::CARD).as_card().nil();
        // and cd.xyz_card:IsFaceup() then
        if card::is_relate_to_effect(d, xyz, a.e) && card::is_faceup(d, xyz) {
            // Duel.Damage(1-tp,cd.xyz_card:GetRank()*300,REASON_EFFECT)
            let dmg = card::get_rank(d, xyz) as i64 * 300;
            duel::damage(d, 1 - a.tp, dmg, REASON_EFFECT, false, None);
        }
    }
}
