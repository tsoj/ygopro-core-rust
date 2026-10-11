//! Gishki Nekromirror
use super::prelude::*;

pub const ID: u32 = 36982581;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GISHKI],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return to the top and bottom of the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `Ritual.Target(rparams)` / `Ritual.Operation(rparams)`: `{filter=aux.FilterBoolFunction(Card.IsSetCard,SET_GISHKI), lvtype=RITPROC_EQUAL, stage2=s.stage2}`
fn ritual_params() -> ritual::RitualProc {
    ritual::RitualProc {
        filter: Some(std::sync::Arc::new(|d, c| card::is_set_card(d, c, &[SET_GISHKI as u16]))),
        lvtype: RITPROC_EQUAL,
        stage2: Some(std::sync::Arc::new(stage2)),
        ..Default::default()
    }
}
/// `s.stage2(mg,e,tp,eg,ep,ev,re,r,rp,sc)`
fn stage2(d: &mut Duel, _mg: &CardSet, a: &EffArgs, sc: CardId) {
    // Duel.SetLP(tp,Duel.GetLP(tp)-sc:GetBaseAttack())
    let lp = duel::get_lp(d, a.tp) - card::get_base_attack(d, sc);
    duel::set_lp(d, a.tp, lp);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRitualMonster() and c:IsSetCard(SET_GISHKI) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_RITUAL,tp,false,true)
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,0,LOCATION_MZONE,1,nil,e,c)
    card::is_ritual_monster(d, c)
        && card::is_set_card(d, c, &[SET_GISHKI as u16])
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_RITUAL, tp, false, true, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, cc| cfilter(d, cc, e, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.cfilter(c,e,sc)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, sc: CardId) -> bool {
    // return c:IsFaceup() and c:IsCanBeRitualMaterial(sc) and not c:IsImmuneToEffect(e)
    card::is_faceup(d, c) && card::is_can_be_ritual_material(d, c, Some(sc), None) && !card::is_immune_to_effect(d, c, e)
}
/// The arguments `e,tp,eg,ep,ev,re,r,rp` as the core passes them, plus `chk` for a target
fn eff_values(a: &EffArgs, chk: Option<i64>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        v.push(Value::Int(chk));
    }
    v
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local rittg=Ritual.Target(rparams)
    let rittg = ritual::target(d, ritual_params());
    // if chk==0 then return (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp))
    //     or rittg(e,tp,eg,ep,ev,re,r,rp,chk) end
    if a.chk == Some(0) {
        let b = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
        if b {
            return true;
        }
        return rittg.call(d, &eff_values(a, Some(0))).truthy();
    }
    // rittg(e,tp,eg,ep,ev,re,r,rp,chk)
    rittg.call(d, &eff_values(a, a.chk.map(|v| v as i64))).truthy();
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local rittg,ritop=Ritual.Target(rparams),Ritual.Operation(rparams)
    let rittg = ritual::target(d, ritual_params());
    let ritop = ritual::operation(d, ritual_params());
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    // local b2=rittg(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = rittg.call(d, &eff_values(a, Some(0))).truthy();
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // if op==1 then
    if op == Some(1) {
        // Tribute 1 opponent's face-up monster
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &g) else {
            return;
        };
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local rg=Duel.SelectMatchingCard(tp,s.cfilter,tp,0,LOCATION_MZONE,1,1,nil,e,sc)
        let rg = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, a.e, sc)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #rg==0 then return end
        if rg.is_empty() {
            return;
        }
        // sc:SetMaterial(rg)
        card::set_material(d, sc, rg.clone());
        // Duel.ReleaseRitualMaterial(rg)
        duel::release_ritual_material(d, &rg, false);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SpecialSummon(sc,SUMMON_TYPE_RITUAL,tp,tp,false,true,POS_FACEUP)==0 then return end
        if duel::special_summon(d, sc, SUMMON_TYPE_RITUAL, tp, tp, false, true, POS_FACEUP as u8, None) == 0 {
            return;
        }
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
        // Duel.SetLP(tp,Duel.GetLP(tp)-sc:GetBaseAttack())
        let lp = duel::get_lp(d, tp) - card::get_base_attack(d, sc);
        duel::set_lp(d, tp, lp);
    } else if op == Some(2) {
        // Tribute monsters from your hand or field
        // ritop(e,tp,eg,ep,ev,re,r,rp)
        ritop.call(d, &eff_values(a, None));
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GISHKI) and c:IsMonster() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_GISHKI as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToDeck() and Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, c) && duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g+c,2,tp,0)
    let mut gc = g;
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(gc), 2, tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)>0
    //     and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA)) then return end
    if !(card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA))
    {
        return;
    }
    // if tc:IsLocation(LOCATION_DECK) then Duel.ConfirmDecktop(tp,1) end
    if card::is_location(d, tc, LOCATION_DECK) {
        duel::confirm_decktop(d, tp, 1);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoDeck(c,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
