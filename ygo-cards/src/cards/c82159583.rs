//! Clown Crew Biancaviso
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 82159583;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Tribute Summon this card face-up by Tributing 1 Ritual, Fusion, Synchro, Xyz, Pendulum, or Link Monster
    // local e0=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.tributefilter)
    let _e0 = aux::add_normal_summon_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE),
        Some(aux::stringid(ID, 0)),
        Some(std::sync::Arc::new(tributefilter)),
        None,
    );
    // If this card is Tribute Summoned: You can activate 1 of these effects (but you can only use each of these effects of "Clown Crew Biancaviso" once per turn)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(function(e) return c:IsTributeSummoned() end)
    effect::set_condition(d, e1, Some(lua_fn(move |d, _a: Args| card::is_tribute_summoned(d, c))));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn, during your opponent's Main Phase, you can (Quick Effect): Immediately after this effect resolves, Tribute Summon 1 monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_SUMMON+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SUMMON + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsMainPhase(1-tp) end)
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| duel::is_main_phase(d, Some(1 - a.p(1))))));
    // e2:SetTarget(s.tribsumtg)
    effect::set_target(d, e2, Some(cost_target(tribsumtg)));
    // e2:SetOperation(s.tribsumop)
    effect::set_operation(d, e2, Some(operation(tribsumop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tributefilter(c,tp)`
fn tributefilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_RITUAL|TYPE_PENDULUM|TYPE_EXTRA) and (c:IsControler(tp) or c:IsFaceup())
    card::is_type(d, c, TYPE_RITUAL | TYPE_PENDULUM | TYPE_EXTRA) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=e:GetHandler():GetMaterialCount()
    let handler = effect::get_handler(d, a.e).nil();
    let ct = card::get_material_count(d, handler) as u32;
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsPlayerCanDraw(tp,ct)
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None) && duel::is_player_can_draw(d, a.tp, ct);
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingMatchingCard(Card.IsNegatable,tp,0,LOCATION_ONFIELD,ct,nil)
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, 0, LOCATION_ONFIELD, ct, NO_EX);
    // if chk==0 then return ct>0 and (b1 or b2) end
    if a.chk == Some(0) {
        return ct > 0 && (b1 || b2);
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,3)},
    //     {b2,aux.Stringid(id,4)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
    // e:SetLabel(op,ct)
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64), ct as i64]);
    // if op==1 then
    if op == Some(1) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, ct as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_DISABLE)
        effect::set_category(d, a.e, CATEGORY_DISABLE);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,ct,1-tp,LOCATION_ONFIELD)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, ct as u8, 1 - a.tp, LOCATION_ONFIELD as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op,ct=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let op = labels.first().copied().unwrap_or(0);
    let ct = labels.get(1).copied().unwrap_or(0);
    // if op==1 then
    if op == 1 {
        // --● Draw cards equal to the number of monsters Tributed for its Tribute Summon
        // Duel.Draw(tp,ct,REASON_EFFECT)
        duel::draw(d, a.tp, ct as u32, REASON_EFFECT);
    // elseif op==2 then
    } else if op == 2 {
        // --● Negate the effects of face-up cards your opponent controls, equal to the number of monsters Tributed for its Tribute Summon
        // local g=Duel.GetMatchingGroup(Card.IsNegatable,tp,0,LOCATION_ONFIELD,nil)
        let mut g = duel::get_matching_group(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
        // if #g>=ct then
        if g.len() as i64 >= ct {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
            // g=g:Select(tp,ct,ct,nil)
            g = group::select(d, &g, a.tp, ct as u16, ct as u16, false, NO_EX).nil();
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // for sc in g:Iter() do
            for sc in group::iter(d, &g) {
                // --Negate their effects
                // sc:NegateEffects(c,nil,true)
                card::negate_effects(d, sc, c, None, true, None);
            }
        }
    }
}
/// `s.tribsumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tribsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.CanSummonOrSet,tp,LOCATION_HAND,0,1,nil,true,nil,1) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::can_summon_or_set(d, c, true, None, 1, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
/// `s.tribsumop(e,tp,eg,ep,ev,re,r,rp)`
fn tribsumop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,Card.CanSummonOrSet,tp,LOCATION_HAND,0,1,1,nil,true,nil,1):GetFirst()
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, c| card::can_summon_or_set(d, c, true, None, 1, None)),
        a.tp,
        LOCATION_HAND,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // Duel.SummonOrSet(tp,sc,true,nil,1)
        duel::summon_or_set(d, a.tp, sc, true, None, 1, None);
    }
}
